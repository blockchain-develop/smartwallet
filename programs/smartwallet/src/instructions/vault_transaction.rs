#![allow(deprecated)]
#![warn(unsafe_op_in_unsafe_fn)]
use crate::errors::WalletError;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use core::str;
use secp256k1::{ecdsa, Message, PublicKey, Secp256k1};
use sha3::Digest;
use solana_program::instruction::Instruction;
use solana_program::msg;
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::program::{invoke, invoke_signed};

use solana_program::sysvar::instructions::{
    load_current_index_checked, load_instruction_at_checked, ID as IX_ID,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VaultTransactionArgs {
    pub owner: Vec<u8>,
    pub signs: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(args: VaultTransactionArgs)]
pub struct VaultTransaction<'info> {
    #[account(
        seeds = [SEED_PREFIX, SEED_WALLET, args.owner.as_slice()],
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
        //
        //let x = args.owner.as_slice();
        //let args_owner = format!("{:x}", &x);
        let args_owner = to_hex_string(&args.owner);
        msg!("parameter owner: {}", args_owner);

        let args_signs = to_hex_string(&args.signs);
        msg!("parameter signs: {}", args_signs);

        let args_data = to_hex_string(&args.data);
        msg!("parameter data: {}", args_data);

        // get the instrction
        let index = load_current_index_checked(&ctx.accounts.ix_sysvar)?;
        let ix = load_instruction_at_checked(index as usize, &ctx.accounts.ix_sysvar)?;
        msg!("instruction index: {}", index);

        // verify signature
        // todo,
        // 1. how to verify signature
        // 2. the data ?
        /*
                check_secp256k1_data(
                    &ix.data,
                    args.owner.as_slice(),
                    b"",
                    args.signs.as_slice(),
                    0,
                )?;
        */

        /*
                let mut hasher = sha3::Keccak256::new();
                hasher.update(ix.data);
                let message_hash = hasher.finalize();

                let secp = Secp256k1::new();
                let message = Message::from_digest(message_hash.into());
                let sign = ecdsa::Signature::from_compact(args.signs.as_slice()).ok();
                let public_key = PublicKey::from_slice(args.owner.as_slice()).ok();
                let verified = secp
                    .verify_ecdsa(&message, &sign.unwrap(), &public_key.unwrap())
                    .is_ok();
        */
        // execute this instruction
        let mut instruction_accounts = vec![];
        let mut nstruction_account_infos = vec![];
        for account_info in ctx.remaining_accounts.iter() {
            instruction_accounts.push(AccountMeta {
                pubkey: *account_info.key,
                is_signer: account_info.is_signer,
                is_writable: account_info.is_writable,
            });
            nstruction_account_infos.push(account_info.clone());
        }

        invoke_signed(
            &Instruction {
                program_id: *ctx.accounts.program.key,
                accounts: instruction_accounts,
                data: args.data,
            },
            &nstruction_account_infos[..],
            &[&[SEED_PREFIX, SEED_WALLET, args.owner.as_slice()]],
            // & [&seeds(args.owner.as_slice())],
        )?;

        Ok(())
    }
}

pub fn seeds(owner: &[u8]) -> [&[u8]; 3] {
    [&SEED_PREFIX, &SEED_WALLET, owner]
}

/// Verify serialized Secp256k1Program instruction data
pub fn check_secp256k1_data(
    data: &[u8],
    pubkey: &[u8],
    msg: &[u8],
    sig: &[u8],
    recovery_id: u8,
) -> Result<()> {
    // According to this layout used by the Secp256k1Program
    // https://github.com/solana-labs/solana-web3.js/blob/master/src/secp256k1-program.ts#L49

    // "Deserializing" byte slices

    let num_signatures = &[data[0]]; // Byte  0
    let signature_offset = &data[1..=2]; // Bytes 1,2
    let signature_instruction_index = &[data[3]]; // Byte  3
    let eth_address_offset = &data[4..=5]; // Bytes 4,5
    let eth_address_instruction_index = &[data[6]]; // Byte  6
    let message_data_offset = &data[7..=8]; // Bytes 7,8
    let message_data_size = &data[9..=10]; // Bytes 9,10
    let message_instruction_index = &[data[11]]; // Byte  11

    let data_eth_address = &data[12..12 + 20]; // Bytes 12..12+20
    let data_sig = &data[32..32 + 64]; // Bytes 32..32+64
    let data_recovery_id = &[data[96]]; // Byte  96
    let data_msg = &data[97..]; // Bytes 97..end

    // Expected values

    const SIGNATURE_OFFSETS_SERIALIZED_SIZE: u16 = 11;
    const DATA_START: u16 = 1 + SIGNATURE_OFFSETS_SERIALIZED_SIZE;

    let msg_len: u16 = msg.len().try_into().unwrap();
    let eth_address_len: u16 = pubkey.len().try_into().unwrap();
    let sig_len: u16 = sig.len().try_into().unwrap();

    let exp_eth_address_offset: u16 = DATA_START;
    let exp_signature_offset: u16 = DATA_START + eth_address_len;
    let exp_message_data_offset: u16 = exp_signature_offset + sig_len + 1;
    let exp_num_signatures: u8 = 1;

    // Header and Arg Checks

    // Header
    if num_signatures != &exp_num_signatures.to_le_bytes()
        || signature_offset != &exp_signature_offset.to_le_bytes()
        || signature_instruction_index != &[0]
        || eth_address_offset != &exp_eth_address_offset.to_le_bytes()
        || eth_address_instruction_index != &[0]
        || message_data_offset != &exp_message_data_offset.to_le_bytes()
        || message_data_size != &msg_len.to_le_bytes()
        || message_instruction_index != &[0]
    {
        return Err(WalletError::Unauthorized.into());
    }

    // Arguments
    if data_eth_address != pubkey
        || data_sig != sig
        || data_recovery_id != &[recovery_id]
        || data_msg != msg
    {
        return Err(WalletError::Unauthorized.into());
    }

    Ok(())
}

pub fn check_ed25519_data(data: &[u8], pubkey: &[u8], msg: &[u8], sig: &[u8]) -> Result<()> {
    // According to this layout used by the Ed25519Program
    // https://github.com/solana-labs/solana-web3.js/blob/master/src/ed25519-program.ts#L33

    // "Deserializing" byte slices

    let num_signatures = &[data[0]]; // Byte  0
    let padding = &[data[1]]; // Byte  1
    let signature_offset = &data[2..=3]; // Bytes 2,3
    let signature_instruction_index = &data[4..=5]; // Bytes 4,5
    let public_key_offset = &data[6..=7]; // Bytes 6,7
    let public_key_instruction_index = &data[8..=9]; // Bytes 8,9
    let message_data_offset = &data[10..=11]; // Bytes 10,11
    let message_data_size = &data[12..=13]; // Bytes 12,13
    let message_instruction_index = &data[14..=15]; // Bytes 14,15

    let data_pubkey = &data[16..16 + 32]; // Bytes 16..16+32
    let data_sig = &data[48..48 + 64]; // Bytes 48..48+64
    let data_msg = &data[112..]; // Bytes 112..end

    // Expected values

    let exp_public_key_offset: u16 = 16; // 2*u8 + 7*u16
    let exp_signature_offset: u16 = exp_public_key_offset + pubkey.len() as u16;
    let exp_message_data_offset: u16 = exp_signature_offset + sig.len() as u16;
    let exp_num_signatures: u8 = 1;
    let exp_message_data_size: u16 = msg.len().try_into().unwrap();

    // Header and Arg Checks

    // Header
    if num_signatures != &exp_num_signatures.to_le_bytes()
        || padding != &[0]
        || signature_offset != &exp_signature_offset.to_le_bytes()
        || signature_instruction_index != &u16::MAX.to_le_bytes()
        || public_key_offset != &exp_public_key_offset.to_le_bytes()
        || public_key_instruction_index != &u16::MAX.to_le_bytes()
        || message_data_offset != &exp_message_data_offset.to_le_bytes()
        || message_data_size != &exp_message_data_size.to_le_bytes()
        || message_instruction_index != &u16::MAX.to_le_bytes()
    {
        msg!("Verification failed!");
        return Err(WalletError::Unauthorized.into());
    }

    // Arguments
    if data_pubkey != pubkey || data_msg != msg || data_sig != sig {
        msg!("Verification failed!");
        return Err(WalletError::Unauthorized.into());
    }

    msg!("Verification succeeded!");

    Ok(())
}

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.connect(" ")
}
