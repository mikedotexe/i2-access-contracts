use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, Env, StdResult};
use cw_access_contract_pkg::msgs::query_msg::QueryMsg;

/// Query entry point
/// See a list of query variants in the [QueryMsg](QueryMsg) enum
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}
