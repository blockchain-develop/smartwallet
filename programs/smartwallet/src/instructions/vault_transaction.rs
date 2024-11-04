#![allow(deprecated)]
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::errors::MultisigError;
use crate::state::*;

use solana_program::sysvar::instructions::{
    load_current_index_checked, load_instruction_at_checked, ID as IX_ID,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VaultTransactionArgs {
    pub signs: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(args: VaultTransactionArgs)]
pub struct VaultTransaction<'info> {
    #[account(
        seeds = [SEED_PREFIX, SEED_MULTISIG, owner.key().as_ref()],
        bump
    )]
    pub wallet: Account<'info, Wallet>,

    /// CHECK:
    pub owner: AccountInfo<'info>,

    /// The creator of the multisig.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK:
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}

impl VaultTransaction<'_> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Creates a multisig.
    #[access_control(ctx.accounts.validate())]
    pub fn execute(ctx: Context<Self>, args: VaultTransactionArgs) -> Result<()> {
        //
        let index = load_current_index_checked(&ctx.accounts.ix_sysvar)?;
        let instruction = load_instruction_at_checked(index as usize, &ctx.accounts.ix_sysvar)?;

        // Initialize the multisig.
        // Initialize the multisig.
        let multisig = &mut ctx.accounts.wallet;
        multisig.transaction_index = 0;
        multisig.create_key = ctx.accounts.owner.key();
        multisig.bump = ctx.bumps.wallet;

        //

        Ok(())
    }
}
