//! The errors that can be thrown for this boolean contract, including demonstration ones.

use cosmwasm_std::StdError;
use croncat_errors_macro::croncat_error;
use thiserror::Error;

#[croncat_error]
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Only Wasm Executes allowed")]
    NotSupported,

    #[error("Received invalid CosmosMsg")]
    InvalidCosmosMsg,

    #[error("Allowed callers contains invalid address")]
    InvalidAllowedCaller,

    #[error("Sender is not among the allowed callers")]
    Unauthorized,

    #[error("Contract specified does not match target contract intended for this access contract")]
    WrongContract,

    #[error("Method not allowed on this contract")]
    InvalidMethod,
}
