use cosmwasm_schema::write_api;

use cw_access_contract::msgs::ExecuteMsg;
use cw_access_contract::msgs::InstantiateMsg;
use cw_access_contract::msgs::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
