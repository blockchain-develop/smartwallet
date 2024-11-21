use crate::errors::WalletError;
use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;
use solana_program::msg;
use solana_program::program::invoke_signed;
use solana_program::pubkey::Pubkey;
use solana_program::secp256k1_recover;
use solana_program::sysvar::instructions::{
    load_current_index_checked, load_instruction_at_checked, ID as IX_ID,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VaultTransactionArgs {
    pub owner: Vec<u8>,
    pub signs: Vec<u8>,
    pub recovery_id: u8,
    pub data: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(args: VaultTransactionArgs)]
pub struct VaultTransaction<'info> {
    #[account(
        seeds = [SEED_WALLET, SEED_CONFIG, args.owner.as_slice()],
        bump
    )]
    pub wallet: Account<'info, Wallet>,

    /// The creator of the multisig.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK:
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,

    /// CHECK:
    pub program: AccountInfo<'info>,
}

impl VaultTransaction<'_> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Creates a multisig.
    #[access_control(ctx.accounts.validate())]
    pub fn execute(ctx: Context<Self>, args: VaultTransactionArgs) -> Result<()> {
        msg!("execute");
        //
        let args_owner = to_hex_string(&args.owner);
        msg!("parameter owner: {}", args_owner);

        let args_signs = to_hex_string(&args.signs);
        msg!("parameter signs: {}", args_signs);

        let args_data = to_hex_string(&args.data);
        msg!("parameter data: {}", args_data);

        // todo
        // the data of sign
        // reploy - transaction_index

        // verify the sec256k1 signature
        //

        // get the instrction
        let index = load_current_index_checked(&ctx.accounts.ix_sysvar)?;
        let ix = load_instruction_at_checked(index as usize, &ctx.accounts.ix_sysvar)?;
        msg!("instruction index: {}", index);

        let message_hash = {
            let mut hasher = solana_program::keccak::Hasher::default();
            hasher.hash(&args.data.as_slice());
            hasher.result()
        };
        msg!("msg hash: {}", to_hex_string(&message_hash.try_to_vec()?));

        let public_key = match secp256k1_recover::secp256k1_recover(
            message_hash.try_to_vec()?.as_slice(),
            args.recovery_id,
            args.signs.as_slice(),
        ) {
            Ok(k) => k,
            Err(e) => {
                msg!("{}", e);
                return Err(WalletError::InvalidSignature.into());
            }
        };
        let mut com_public_key = public_key.try_to_vec()?;
        com_public_key.resize(32, 0);
        msg!("recover public key: {}", to_hex_string(&com_public_key));

        if com_public_key.as_slice() != args.owner.as_slice() {
            return Err(WalletError::InvalidSignature.into());
        }

        // execute this instruction
        //

        let seeds_owner = args.owner.as_slice();
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
        let mut instruction_accounts = vec![];
        let mut instruction_account_infos = vec![];
        for account_info in ctx.remaining_accounts.iter() {
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
        }
        invoke_signed(
            &Instruction {
                program_id: *ctx.accounts.program.key,
                accounts: instruction_accounts,
                data: args.data,
            },
            &instruction_account_infos[..],
            &[seeds],
        )?;

        Ok(())
    }
}
