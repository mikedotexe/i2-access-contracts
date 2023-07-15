use crate::errors::ContractError;
use crate::state::{ALLOWED_CALLERS, ALLOWED_METHODS, CREATOR, CRONCAT_FACTORY_CONTRACT, TARGET_CONTRACT};
use cosmwasm_std::{Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, StdResult, WasmMsg};
use croncat_integration_utils::task_creation::get_latest_croncat_contract;
use osmosis_std::shim::Any;
use osmosis_std::types::cosmos::authz::v1beta1::MsgExec;
use osmosis_std::types::cosmos::base::v1beta1::Coin as ProtoCoin;
use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContract;
use serde_cw_value::Value;

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cosmos_msg: CosmosMsg,
) -> Result<Response, ContractError> {
    match cosmos_msg {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr,
            msg,
            funds,
        }) => {
            let funds: Vec<ProtoCoin> = funds
                .into_iter()
                .map(|c| ProtoCoin {
                    denom: c.denom,
                    amount: c.amount.to_string(),
                })
                .collect();

            // Ensure the contract address is real and the target contract
            let contract_addr = deps.api.addr_validate(&contract_addr)?;
            if contract_addr != TARGET_CONTRACT.load(deps.storage)? {
                return Err(ContractError::WrongContract);
            }

            // Get latest CronCat manager contract address
            // This is the one that'll be sending a message here.
            // We *could* look at past manager contracts as well,
            // but this is likely the safest.
            let croncat_factory_address_opt = CRONCAT_FACTORY_CONTRACT.may_load(deps.storage)?;

            // Ensure the caller is allowed
            if !ALLOWED_CALLERS.has(deps.storage, info.sender.clone()) {
            // if !ALLOWED_CALLERS.has(deps.storage, info.sender.clone()) && info.sender != croncat_manager_address {
                if let Some(croncat_factory_address) = croncat_factory_address_opt {
                    let croncat_manager_address: Addr = get_latest_croncat_contract(&deps.querier, croncat_factory_address, "manager".to_string())?;
                    if info.sender != croncat_manager_address {
                        return Err(ContractError::Unauthorized);
                    }
                } else {
                    return Err(ContractError::Unauthorized);
                }
            }

            // Turn this into a JSON Value basically
            let msg_value: Value = bin_to_value(&msg)?;
            let Value::Map(mut method_name_map) = msg_value else {
                return Err(ContractError::InvalidCosmosMsg);
            };

            // Being overcareful. Likely don't need this.
            // Only accept if there's one top-level key
            if method_name_map.len() != 1 {
                return Err(ContractError::InvalidCosmosMsg);
            }

            let only_msg = method_name_map.pop_first();

            if only_msg.is_none() {
                return Err(ContractError::InvalidCosmosMsg);
            }

            let method_map_value = only_msg.unwrap().0;
            let Value::String(method_name) = method_map_value else {
                return Err(ContractError::InvalidCosmosMsg)
            };

            // Check to see if the method name is in the allowed set
            if !ALLOWED_METHODS.has(deps.storage, method_name.clone()) {
                return Err(ContractError::InvalidMethod);
            }

            // Note that in this example we only check the top-level key
            // To dig deeper, look at this section:
            // https://github.com/CosmWasm/serde-cw-value#usage
            // Thank you, Bartłomiej Kuras

            // Ready for authz execute!
            // (Since we've passed all the custom checks)

            let creator = CREATOR.load(deps.storage)?;

            let msg_execute_contract = MsgExecuteContract {
                sender: creator.to_string(),
                contract: contract_addr.to_string(),
                msg: msg.to_vec(),
                funds,
            };

            let msg_execute_contract_any = Any {
                type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".to_string(),
                value: msg_execute_contract.to_proto_bytes(),
            };

            let msg_exec = MsgExec {
                grantee: env.contract.address.to_string(),
                msgs: vec![msg_execute_contract_any],
            };

            Ok(Response::new()
                .add_attribute("action", "exec")
                .add_attribute("method_name", method_name)
                .add_message(msg_exec))
        }
        _ => Err(ContractError::NotSupported),
    }
}

// Taken from CronCat's mod-generic
// https://docs.rs/croncat-mod-generic/1.0.3/croncat_mod_generic/helpers/fn.bin_to_value.html
fn bin_to_value(bin: &[u8]) -> StdResult<Value> {
    cosmwasm_std::from_slice(bin)
        .map_err(|e| StdError::parse_err(std::any::type_name::<Value>(), e))
}
