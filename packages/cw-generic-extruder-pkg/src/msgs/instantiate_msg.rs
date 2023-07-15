use cosmwasm_schema::cw_serde;

/// The instantiate message with no params
// NOTE: Half-considering having an optional owner, but kind of like that it's fixed to be the sender, honestly.
#[cw_serde]
pub struct InstantiateMsg {}
