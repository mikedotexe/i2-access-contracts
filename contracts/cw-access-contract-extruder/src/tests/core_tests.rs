/*
use crate::tests::{alice, contracts, default_app};
use cw_i2_creator_pkg::msgs::instantiate_msg::InstantiateMsg;
use cw_multi_test::Executor;
use cw_generic_extruder_pkg::msgs::instantiate_msg::InstantiateMsg;

#[test]
fn the_usual() {
    let mut app = default_app();
    let code_id = app.store_code(contracts::me());
    assert!(
        app.instantiate_contract(
            code_id,
            alice(), // from
            &InstantiateMsg { code_id, checksum: "028d2726311d36a899868afc4162752fdd098019fa1577e1c29181a28ba5f733".to_string() },
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
// Looks like multi-test doesn't know about Instantiate2 at the time of this writing
#[ignore]
fn call_create_simple() {
    let mut app = default_app();

    // Deploy/store base contract
    let base_code_id = app.store_code(BaseI2Contract::me());

    let code_id = app.store_code(contracts::me());
    let creator_addr = app.instantiate_contract(
            code_id,
            alice(), // from
            &InstantiateMsg { code_id: base_code_id, checksum: "028d2726311d36a899868afc4162752fdd098019fa1577e1c29181a28ba5f733".to_string() },
            &[], // funds
            "regifted the labelmaker",
            None, // No admin, so it'll be immutable
        ).expect("Should be able to instantiate fine");
    println!("aloha creator_addr {:?}", creator_addr);

    let exec_res = app.execute_contract(alice(), creator_addr, &cw_i2_creator_pkg::msgs::execute_msg::ExecuteMsg::Create {
        target_contract: "juno178zy27yelrnr8d86u8q5yhgtr7d6jfyadrrsw9kqr78ykxh897gssas2v2".to_string(),
        allowed_methods: vec!("method1".to_string(), "method2".to_string()),
        delta: 0,
    }, &[]);

    println!("exec_res {:?}", exec_res);
    assert!(exec_res.is_ok(), "ya fucked up");
}

 */
