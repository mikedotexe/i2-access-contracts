use cosmwasm_schema::write_api;

use cw_access_contract_extruder::msgs::ExecuteMsg;
use cw_access_contract_extruder::msgs::InstantiateMsg;
use cw_access_contract_extruder::msgs::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
