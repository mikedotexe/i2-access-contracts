use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin};

/// Lists the available execute messages for this contract
#[cw_serde]
pub enum ExecuteMsg {
    Create {
        /// The contract the signer wishes to call, through this contract
        /// The authz execute will only happen if the key is allowed to
        /// call the given method, as defined by access control
        code_id: u64,
    },
}
