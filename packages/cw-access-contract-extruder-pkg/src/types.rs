use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, HexBinary, Binary};

#[cw_serde]
pub struct Config {
    /// Some, most, or all methods check this address for privileged access
    pub owner: String,
}

#[cw_serde]
pub struct ExtruderTypeGeneric {
    pub ty: String,
    pub data: Option<Binary>,
}

#[cw_serde]
pub struct SaltTopLevelKeyExtruder {
    pub extruder: ExtruderTypeGeneric,
}
