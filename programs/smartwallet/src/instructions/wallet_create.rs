#![allow(deprecated)]
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::errors::WalletError;
use crate::state::*;
use solana_program::pubkey::Pubkey;
use std::str;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WalletCreateArgs {
    pub owner: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(args: WalletCreateArgs)]
pub struct WalletCreate<'info> {
    #[account(
        init,
        payer = creator,
        space = Wallet::size(),
        seeds = [SEED_WALLET, SEED_CONFIG, args.owner.as_slice()],
        bump
    )]
    pub wallet: Account<'info, Wallet>,

    /// The creator of the wallet.
    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl WalletCreate<'_> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Creates a multisig.
    #[access_control(ctx.accounts.validate())]
    pub fn create(ctx: Context<Self>, args: WalletCreateArgs) -> Result<()> {
        //
        let args_owner = to_hex_string(&args.owner);
        msg!("parameter owner: {}", args_owner);

        let owner_seeds = args.owner.as_slice();
        msg!("{}", owner_seeds.len());
        // Initialize the multisig.
        let seeds = &[SEED_WALLET, SEED_OWNER, owner_seeds];
        let (pda, bump) = Pubkey::find_program_address(seeds, ctx.program_id);
        msg!("new wallet owner address: {}", pda);
        msg!("new wallet bump: {}", bump);
        let wallet = &mut ctx.accounts.wallet;
        wallet.transaction_index = 0;
        wallet.owner = args.owner;
        wallet.bump = bump;

        Ok(())
    }
}

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.connect("")
}
