use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct SaltItems {
    pub target_contract: String,
    pub allowed_methods: Vec<String>,
    pub delta: u32,
}
