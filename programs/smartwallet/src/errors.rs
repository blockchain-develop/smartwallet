use anchor_lang::prelude::*;

#[error_code]
pub enum WalletError {
    #[msg("Attempted to perform an unauthorized action")]
    Unauthorized,
    #[msg("TransactionMessage is malformed.")]
    InvalidTransactionMessage,
}
