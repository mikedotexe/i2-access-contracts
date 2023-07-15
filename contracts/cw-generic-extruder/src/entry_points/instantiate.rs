use crate::errors::ContractError;
use crate::state::{Config, CONFIG};
use crate::{CONTRACT_NAME, CONTRACT_VERSION};
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use cw_generic_extruder_pkg::msgs::instantiate_msg::InstantiateMsg;

/// Instantiate entry point
/// See the instantiate message and fields in [InstantiateMsg](InstantiateMsg)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender.to_string(),
    };
    CONFIG.save(deps.storage, &config)?;

    // Sets the version via cw2, just a normal thing to do
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}
