use cosmwasm_schema::cw_serde;
use cosmwasm_std::CosmosMsg;

/// Lists the available execute messages for this contract
#[cw_serde]
pub enum ExecuteMsg {
    Exec { cosmos_msg: CosmosMsg },
}
