use super::check_ed25519_data;
use crate::check_secp256k1_data;
use crate::errors::WalletError;
use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use solana_program::ed25519_program::ID as ED25519_ID;
use solana_program::instruction::Instruction;
use solana_program::msg;
use solana_program::program::invoke_signed;
use solana_program::pubkey::Pubkey;
use solana_program::secp256k1_program::ID as SECP256K1_ID;
use solana_program::sysvar::instructions::{
    load_current_index_checked, load_instruction_at_checked, ID as IX_ID,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InstructionArgs {
    pub account_size: u8,
    pub data: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VaultTransactionArgs {
    pub wallet_id: Vec<u8>,
    pub verifiers: Vec<u8>,
    pub instructions: Vec<InstructionArgs>,
}

#[derive(Accounts)]
#[instruction(args: VaultTransactionArgs)]
pub struct VaultTransaction<'info> {
    #[account(
        mut,
        seeds = [SEED_WALLET, SEED_CONFIG, args.wallet_id.as_slice()],
        bump
    )]
    pub wallet: Account<'info, Wallet>,

    /// CHECK:
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,

    /// The creator of the multisig.
    #[account(mut)]
    pub payer: Signer<'info>,
}

impl VaultTransaction<'_> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Creates a multisig.
    #[access_control(ctx.accounts.validate())]
    pub fn execute(ctx: Context<Self>, args: VaultTransactionArgs) -> Result<()> {
        msg!("execute");
        //  parameters
        //
        let args_owner = to_hex_string(&args.wallet_id);
        msg!("parameter owner: {}", args_owner);

        for (i, verifier) in args.verifiers.iter().enumerate() {
            msg!("verifier number: {}", i);

            msg!("verifier id: {}", verifier);
        }

        for (i, instruction) in args.instructions.iter().enumerate() {
            msg!("instrunction number: {}", i);

            msg!("instrunction account size: {}", instruction.account_size);

            let instruction_data = to_hex_string(&instruction.data);
            msg!("instrunction data: {}", instruction_data);
        }

        // the smart wallet owner address
        //
        let seeds_owner = args.wallet_id.as_slice();
        msg!("{}", seeds_owner.len());
        let seeds = &[
            SEED_WALLET,
            SEED_OWNER,
            seeds_owner,
            &[ctx.accounts.wallet.bump],
        ];
        let smart_wallet_owner_address = match Pubkey::create_program_address(seeds, ctx.program_id)
        {
            Ok(address) => address,
            Err(e) => {
                msg!("{}", e);
                return Err(WalletError::CannotCreateProgramAddress.into());
            }
        };

        // try to rebuild all instructions
        //
        let mut insturctions = vec![];
        let mut instructions_accounts = vec![];
        let mut raw_instructions = vec![];
        let mut account_index = 0;
        for instruction in args.instructions.iter() {
            let mut instruction_accounts = vec![];
            let mut instruction_account_infos = vec![];
            let program_id = *ctx.remaining_accounts.get(account_index).unwrap().key;
            account_index += 1;
            for _i in 0..instruction.account_size {
                let account_info = ctx.remaining_accounts.get(account_index).unwrap();
                if *account_info.key == smart_wallet_owner_address {
                    instruction_accounts.push(AccountMeta {
                        pubkey: *account_info.key,
                        is_signer: true,
                        is_writable: account_info.is_writable,
                    });
                } else {
                    instruction_accounts.push(AccountMeta {
                        pubkey: *account_info.key,
                        is_signer: account_info.is_signer,
                        is_writable: account_info.is_writable,
                    });
                }
                instruction_account_infos.push(account_info.clone());
                account_index += 1;
            }
            let instruction = Instruction {
                program_id: program_id,
                accounts: instruction_accounts,
                data: instruction.data.clone(),
            };
            let instruction_raw = bincode::serialize(&instruction).unwrap();
            //
            insturctions.push(instruction);
            raw_instructions.push(instruction_raw);
            instructions_accounts.push(instruction_account_infos);
        }

        // build signed raw
        //
        let mut transaction_index = ctx.accounts.wallet.transaction_index.try_to_vec()?;
        let mut raw = vec![];
        for raw_instruction in raw_instructions.iter_mut() {
            raw.append(raw_instruction);
        }
        raw.append(&mut transaction_index);
        msg!("raw instruction: {}", to_hex_string(&raw));

        // the hash of raw
        //
        let message_hash = {
            let mut hasher = solana_program::keccak::Hasher::default();
            hasher.hash(&raw.as_slice());
            hasher.result()
        };
        msg!("msg hash: {}", to_hex_string(&message_hash.try_to_vec()?));

        // verify signature
        //
        let current_index = load_current_index_checked(&ctx.accounts.ix_sysvar)?;
        let signature_verify_index = current_index - 1;
        let signature_verify_instruction =
            load_instruction_at_checked(signature_verify_index as usize, &ctx.accounts.ix_sysvar)?;
        let result = ctx
            .accounts
            .wallet
            .owner
            .iter()
            .find(|&x| x.id == args.verifiers[0]);

        let verifier = match result {
            Some(x) => x,
            None => {
                msg!("unknown verifier");
                return Err(WalletError::InvalidSignature.into());
            }
        };

        let _verified = match Self::verify(
            &signature_verify_instruction,
            &verifier,
            message_hash.try_to_vec()?.as_slice(),
        ) {
            Ok(_) => true,
            Err(e) => {
                msg!("{}", e);
                return Err(WalletError::InvalidSignature.into());
            }
        };

        // execute this instruction
        //
        for i in 0..raw_instructions.len() {
            let instruction = insturctions.get(i).unwrap();
            let instruction_account_infos = instructions_accounts.get(i).unwrap();
            invoke_signed(&instruction, &instruction_account_infos[..], &[seeds])?;
        }

        // nonce
        let wallet = &mut ctx.accounts.wallet;
        wallet.transaction_index += 1;
        msg!("new transaction index: {}", wallet.transaction_index);

        Ok(())
    }

    fn verify(ix: &Instruction, verifier: &Verifier, hash: &[u8]) -> Result<()> {
        match ix.program_id {
            ED25519_ID => check_ed25519_data(&ix.data, &verifier.key.as_slice(), hash)?,
            SECP256K1_ID => check_secp256k1_data(&ix.data, verifier.key.as_slice(), hash)?,
            _ => {
                msg!("unknown verifier");
                return Err(WalletError::InvalidSignature.into());
            }
        }
        Ok(())
    }
}
