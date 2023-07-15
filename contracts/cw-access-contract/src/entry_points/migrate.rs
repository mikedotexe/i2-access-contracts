use cosmwasm_std::{entry_point, DepsMut, Env, Response, StdResult};
use cw_access_contract_pkg::msgs::migrate_msg::MigrateMsg;

/// This is the most important method, which allows the
/// contract to be overwritten after a named contract is created
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
