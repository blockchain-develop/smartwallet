use anchor_lang::prelude::*;

#[error_code]
pub enum WalletError {
    #[msg("Attempted to perform an unauthorized action.")]
    Unauthorized,
    #[msg("Cannot create program address.")]
    CannotCreateProgramAddress,
    #[msg("Signature is invalid.")]
    InvalidSignature,
}
