use std::cmp::max;

use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::errors::*;
use crate::id;

pub const MAX_TIME_LOCK: u32 = 3 * 30 * 24 * 60 * 60; // 3 months

#[account]
pub struct Wallet {
    /// Key that is used to seed the multisig PDA.
    pub owner: Vec<u8>,
    /// Last transaction index. 0 means no transactions have been created.
    pub transaction_index: u64,
    /// Bump for the multisig PDA seed.
    pub bump: u8,
}

impl Wallet {
    pub fn size() -> usize {
        8  + // anchor account discriminator
        32 + 1 + // create_key
        8  + // transaction_index
        1 +  // bump
        32
    }
}
