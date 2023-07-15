//! Execute my logic

use cosmwasm_schema::cw_serde;
use crate::errors::ContractError;
use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response, to_binary, WasmMsg};
use sha2::{Sha512, Digest};
use cw_generic_extruder_pkg::types::{ExtruderTypeGeneric, SaltTopLevelKeyExtruder};

#[cw_serde]
pub struct BlankInstantiate {}

/// Logic for the [MyExecute](cw_i2_creator_pkg::msgs::execute_msg::ExecuteMsg::MyExecute) (`my_execute`) method
pub fn execute(
    _env: Env,
    _deps: DepsMut,
    info: MessageInfo,
    code_id: u64,
) -> Result<Response, ContractError> {
    let salt_struct = SaltTopLevelKeyExtruder {
        extruder: ExtruderTypeGeneric {
            ty: "generic".to_string(),
            data: None,
        },
    };
    let salt_string = serde_json::to_vec(&salt_struct)?;

    // This outputs 64 bytes, which is the max length of the salt
    let mut hasher = Sha512::new();
    hasher.update(salt_string);
    let salt_arr = hasher.finalize();
    let salt = salt_arr.to_vec();
    // println!("salt {:?}", salt);

    // Create Instantiate2 message to send with response
    let instantiate_done_contract_msg = WasmMsg::Instantiate2 {
        admin: Some(info.sender.to_string()),
        code_id,
        label: "Clever Instantiate2".to_string(),
        msg: to_binary(&BlankInstantiate {})?,
        // Just pass along any funds to the new account
        funds: info.funds,
        salt: salt.into(),
    };

    Ok(Response::new().add_attribute("action", "create").add_message(instantiate_done_contract_msg))
}
