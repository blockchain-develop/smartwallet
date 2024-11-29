#![allow(deprecated)]
use anchor_lang::prelude::*;

use crate::state::*;
use crate::utils::*;
use solana_program::pubkey::Pubkey;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WalletCreateArgs {
    pub wallet_id: Vec<u8>,
    pub verifiers: Vec<Verifier>,
}

#[derive(Accounts)]
#[instruction(args: WalletCreateArgs)]
pub struct WalletCreate<'info> {
    #[account(
        init,
        payer = creator,
        space = Wallet::size(),
        seeds = [SEED_WALLET, SEED_CONFIG, args.wallet_id.as_slice()],
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

    #[access_control(ctx.accounts.validate())]
    pub fn create(ctx: Context<Self>, args: WalletCreateArgs) -> Result<()> {
        msg!("create");

        let args_owner = to_hex_string(&args.wallet_id);
        msg!("parameter owner: {}", args_owner);

        // todo
        // this wallet is created?

        let seeds_owner = args.wallet_id.as_slice();
        msg!("{}", seeds_owner.len());

        let seeds = &[SEED_WALLET, SEED_OWNER, seeds_owner];
        let (wallet_owner_address, wallet_owner_bump) =
            Pubkey::find_program_address(seeds, ctx.program_id);
        msg!("wallet owner address: {}", wallet_owner_address);
        msg!("wallet owner bump: {}", wallet_owner_bump);

        // Initialize the wallet info.
        let wallet = &mut ctx.accounts.wallet;
        wallet.transaction_index = 0;
        wallet.owner = args.verifiers;
        wallet.bump = wallet_owner_bump;

        Ok(())
    }
}
