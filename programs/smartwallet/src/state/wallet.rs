use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Verifier {
    pub id: u8,
    pub key: Vec<u8>,
}

#[account]
pub struct Wallet {
    /// Key that is used to seed the PDA.
    pub owner: Vec<Verifier>,
    /// Last transaction index. 0 means no transactions have been created.
    pub transaction_index: u64,
    /// Bump for the wallet PDA seed.
    pub bump: u8,
}

impl Wallet {
    pub fn size() -> usize {
        8  + // anchor account discriminator
        (64 + 1) * 3 + // create_key
        8  + // transaction_index
        1 +  // bump
        32
    }
}
