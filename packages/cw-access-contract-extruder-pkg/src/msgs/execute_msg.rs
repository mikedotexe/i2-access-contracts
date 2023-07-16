use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin, Uint64};

/// Lists the available execute messages for this contract
#[cw_serde]
pub enum ExecuteMsg {
    Create {
        /// The code ID to the Access Contract, typically
        code_id: Uint64,
        target_contract: String,
        allowed_methods: Vec<String>,
        /// In case the user wants to make multiple contracts that are allowed to call the same contract at the same methods
        delta: u32, // u32 is more than enough and don't need to stringify
    },
}
