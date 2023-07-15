use crate::types::SaltItems;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub birth_salt: SaltItems,
    /// If you wish to enable CronCat automation on your Access Contract
    pub croncat_factory_address: Option<String>,
    pub allowed_callers: Option<Vec<String>>,
}
