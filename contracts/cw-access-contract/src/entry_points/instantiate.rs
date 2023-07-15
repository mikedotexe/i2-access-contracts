use crate::errors::ContractError;
use crate::state::{ALLOWED_CALLERS, ALLOWED_METHODS, BIRTH_SALT, CREATOR, CRONCAT_FACTORY_CONTRACT, TARGET_CONTRACT};
use cosmwasm_std::{entry_point, Empty};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_access_contract_pkg::msgs::instantiate_msg::InstantiateMsg;

/// Instantiate entry point
/// See the instantiate message and fields in [InstantiateMsg](InstantiateMsg)
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    CREATOR.save(deps.storage, &info.sender)?;
    BIRTH_SALT.save(deps.storage, &msg.birth_salt)?;
    // Go ahead and add the creator to the allowed callers
    ALLOWED_CALLERS.save(deps.storage, info.sender, &Empty {})?;
    // If they included the CronCat factory address, save it
    if let Some(croncat_factory) = msg.croncat_factory_address {
        let croncat_factory_addr = deps.api.addr_validate(&croncat_factory)?;
        CRONCAT_FACTORY_CONTRACT.save(deps.storage, &croncat_factory_addr)?;
    }
    // Now add the rest if they provided any
    if let Some(allowed_callers) = msg.allowed_callers {
        for address in allowed_callers {
            let valid_addr = deps
                .api
                .addr_validate(&address)
                .map_err(|_err| ContractError::InvalidAllowedCaller)?;
            ALLOWED_CALLERS.save(deps.storage, valid_addr, &Empty {})?;
        }
    }
    // Add allowed methods to set
    for method in msg.birth_salt.allowed_methods {
        ALLOWED_METHODS.save(deps.storage, method, &Empty {})?;
    }

    let target_contract = deps.api.addr_validate(&msg.birth_salt.target_contract)?;
    TARGET_CONTRACT.save(deps.storage, &target_contract)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

/*


pub const TARGET_CONTRACT: Item<Addr> = cw_i2_base_pkg::state::TARGET_CONTRACT;

pub const ALLOWED_METHODS: Map<Addr, Empty> = cw_i2_base_pkg::state::ALLOWED_METHODS;
pub const ALLOWED_CALLERS: Map<Addr, Empty> = cw_i2_base_pkg::state::ALLOWED_CALLERS;
pub const CREATOR: Item<Addr> = cw_i2_base_pkg::state::CREATOR;
pub const BIRTH_SALT: Item<SaltItems> = cw_i2_base_pkg::state::BIRTH_SALT;
 */
