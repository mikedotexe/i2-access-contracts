use cw_access_contract_extruder_pkg::types::Config;
use cw_storage_plus::Item;

// Remember the "pub" here, since it's used elsewhere.
pub const CONFIG: Item<Config> = cw_access_contract_extruder_pkg::state::CONFIG;
