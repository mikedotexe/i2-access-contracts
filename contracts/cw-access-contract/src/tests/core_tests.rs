use crate::tests::{alice, contracts, default_app};
use cosmwasm_std::{to_binary, CosmosMsg, WasmMsg};
use cw_access_contract_pkg::msgs::instantiate_msg::InstantiateMsg;
use cw_access_contract_pkg::types::SaltItems;
use cw_multi_test::Executor;
use schemars::_serde_json::json;

#[test]
fn the_usual() {
    let mut app = default_app();
    let code_id = app.store_code(contracts::me());

    assert!(
        app.instantiate_contract(
            code_id,
            alice(), // from
            &InstantiateMsg {
                birth_salt: SaltItems {
                    target_contract:
                        "juno1mc4wfy9unvy2mwx7dskjqhh6v7qta3vqsxmkayclg4c2jude76es0jcp38"
                            .to_string(),
                    allowed_methods: vec!["claim".to_string(), "restake".to_string()],
                    delta: 0,
                },
                allowed_callers: Some(vec![]),
                croncat_factory_address: None,
            },
            &[], // funds
            "regifted the labelmaker",
            None, // No admin, so it'll be immutable
        )
        .is_ok(),
        "Should instantiate fine"
    );

    println!("To see this, you'll have to run the command:");
    println!("cargo test -- --nocapture");
    println!("or 'cargo t' which won't show warnings");
}

#[test]
fn moar() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = default_app();
    let code_id = app.store_code(contracts::me());
    let me_address = app.instantiate_contract(
        code_id,
        alice(), // from
        &InstantiateMsg {
            birth_salt: SaltItems {
                target_contract: "juno1mc4wfy9unvy2mwx7dskjqhh6v7qta3vqsxmkayclg4c2jude76es0jcp38"
                    .to_string(),
                allowed_methods: vec!["claim".to_string(), "restake".to_string()],
                delta: 0,
            },
            allowed_callers: Some(vec![]),
            croncat_factory_address: None,
        },
        &[], // funds
        "regifted the labelmaker",
        None, // No admin, so it'll be immutable
    )?;

    let alice_cosmos_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: "juno1mc4wfy9unvy2mwx7dskjqhh6v7qta3vqsxmkayclg4c2jude76es0jcp38"
            .to_string(),
        msg: to_binary(&json!({
            "claim": {
                "first_param": "costanza"
            }
        }))?,
        // msg: to_binary(&json!({
        //     "claim": {
        //         "first_param": "costanza"
        //     }
        // }))?,
        funds: vec![],
    });

    let alice_cosmos_msg_binary = to_binary(&alice_cosmos_msg)?;
    let alice_calls_her_access_contract = json!({
        "exec": {
            "cosmos_msg": alice_cosmos_msg
            // "cosmos_msg": alice_cosmos_msg_binary
        }
    });

    let alice_calls_her_access_contract_binary = to_binary(&alice_calls_her_access_contract)?;
    println!(
        "aloha alice_calls_her_access_contract_binary: {:?}",
        alice_calls_her_access_contract_binary
    );

    let res = app.execute(
        alice(),
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: me_address.to_string(),
            msg: alice_calls_her_access_contract_binary,
            funds: vec![],
        }),
    );

    if res.is_err() {
        println!("aloha error: {:?}", res.unwrap_err())
    }

    Ok(())
}
