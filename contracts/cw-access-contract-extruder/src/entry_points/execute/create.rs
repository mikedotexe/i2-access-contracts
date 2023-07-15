//! Execute my logic

use cosmwasm_schema::cw_serde;
use crate::errors::ContractError;
use cosmwasm_std::{Addr, Binary, DepsMut, Env, MessageInfo, Response, to_binary, WasmMsg};
use sha2::{Sha512, Digest};
use crate::state::CONFIG;

#[cw_serde]
pub struct BlankInstantiate {}

pub fn execute(
    _env: Env,
    deps: DepsMut,
    info: MessageInfo,
    target_contract: String,
    mut allowed_methods: Vec<String>,
    delta: u32, // More than enough
) -> Result<Response, ContractError> {
    // For determinism's sake, we sort the methods
    allowed_methods.sort();
    let salt_items = SaltItems {
        target_contract,
        allowed_methods,
        delta,
    };
    let salt_string = serde_json::to_vec(&salt_items)?;
    // println!("aloha salt_string {:?}", salt_string);

    // This outputs 64 bytes, which is the max length of the salt
    let mut hasher = Sha512::new();
    hasher.update(salt_string);
    let salt_arr = hasher.finalize();
    let salt = salt_arr.to_vec();
    // println!("salt {:?}", salt);

    // Grab our owner
    let config = CONFIG.load(deps.storage)?;
    // This was saved during instantiate from info.sender
    let owner = Addr::unchecked(config.owner);
    let code_id = config.base_contract_code_id;

    // Access control, so only owner can proceed
    if info.sender != owner {
        return Err(ContractError::Unauthorized { owner });
    }

    // Create Instantiate2 message to send with response
    let instantiate_done_contract_msg = WasmMsg::Instantiate2 {
        admin: Some(info.sender.to_string()),
        code_id,
        label: "Clever Instantiate2".to_string(),
        msg: to_binary(&cw_i2_base_pkg::msgs::instantiate_msg::InstantiateMsg { allowed_callers: vec![], croncat_factory_address: "".to_string() })?,
        funds: info.funds,
        salt: salt.into(),
    };

    Ok(Response::new().add_attribute("action", "my_execute").add_message(instantiate_done_contract_msg))
}
