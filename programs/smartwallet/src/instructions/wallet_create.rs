#![allow(deprecated)]
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::errors::WalletError;
use crate::state::*;
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
        seeds = [SEED_PREFIX, SEED_WALLET, args.owner.as_slice()],
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
        let args_owner = str::from_utf8(args.owner.as_slice()).ok().unwrap();
        msg!("parameter owner: {}", args_owner);
        // Initialize the multisig.
        let wallet = &mut ctx.accounts.wallet;
        wallet.transaction_index = 0;
        wallet.owner = args.owner;
        wallet.bump = ctx.bumps.wallet;

        Ok(())
    }
}
