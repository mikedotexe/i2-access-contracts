use cw_storage_plus::Item;
pub use cw_generic_extruder_pkg::types::Config;

// Remember the "pub" here, since it's used elsewhere.
pub const CONFIG: Item<Config> = cw_generic_extruder_pkg::state::CONFIG;
