use cosmwasm_schema::cw_serde;
use cosmwasm_std::CosmosMsg;

/// Lists the available execute messages for this contract
#[cw_serde]
pub enum ExecuteMsg {
    /// Attempt to "authz execute" a CosmosMsg
    /// Custom contract logic can entirely control whether this happens
    Exec { cosmos_msg: CosmosMsg },
    /*
        TODO: add methods
            - add_caller
            - remove_caller
            - freeze
            - update_croncat_factory
    */
}
