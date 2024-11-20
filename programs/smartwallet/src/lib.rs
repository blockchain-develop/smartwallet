use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("C7iFDZry7Kdg7gc829qBCPEPKKdDkf1gRMxNxDEqeYUX");

#[program]
pub mod smartwallet {
    use super::*;

    pub fn wallet_create(ctx: Context<WalletCreate>, args: WalletCreateArgs) -> Result<()> {
        WalletCreate::create(ctx, args)
    }

    pub fn execute_transaction(
        ctx: Context<VaultTransaction>,
        args: VaultTransactionArgs,
    ) -> Result<()> {
        VaultTransaction::execute(ctx, args)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
