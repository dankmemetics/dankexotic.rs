use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum ExoticError {
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("State is unititialized")]
    UninitializedState,

    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("State is invalid for requested operation")]
    InvalidState,

    #[error("Operation overflowed")]
    Overflow,

    #[error("Invalid Exotic Type")]
    InvalidExotic,

    #[error("Unauthorized Exotic Mint")]
    UnauthorizedMint,
}
impl From<ExoticError> for ProgramError {
    fn from(e: ExoticError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for ExoticError {
    fn type_of() -> &'static str {
        "ExoticError"
    }
}