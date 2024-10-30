use anchor_lang::prelude::*;

declare_id!("GK4YSUwdKdauZbbEvutnn5Swdg1yKaZwYCym5YuaQDPd");

#[program]
pub mod smartwallet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
