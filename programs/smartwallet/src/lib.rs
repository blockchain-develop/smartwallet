use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("GK4YSUwdKdauZbbEvutnn5Swdg1yKaZwYCym5YuaQDPd");

#[program]
pub mod smartwallet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn wallet_create(ctx: Context<WalletCreate>, args: WalletCreateArgs) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
