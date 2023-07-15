use crate::types::SaltItems;
use cosmwasm_std::{Addr, Empty};
use cw_storage_plus::{Item, Map};

// We might as well have one-letter keys like "a" or "n" to save ones and zeroes.

/// The person who did the Instantiate2 on this account. The "parent"
pub const CREATOR: Item<Addr> = Item::new("c");
/// Poor man's set data structure
pub const ALLOWED_CALLERS: Map<Addr, Empty> = Map::new("a");
/// Although this is contained in the birth salt, this
/// makes it easy and efficient to check
pub const ALLOWED_METHODS: Map<String, Empty> = Map::new("m");
/// Again, taking something from the birth salt
pub const TARGET_CONTRACT: Item<Addr> = Item::new("t");
/// CronCat factory address
pub const CRONCAT_FACTORY_CONTRACT: Item<Addr> = Item::new("f");
/// Birth salt, which is the same as the salt used to create this account if
/// done via Instantiate2. (See cw-i2-creator, AKA Access Contract Extruder)
pub const BIRTH_SALT: Item<SaltItems> = Item::new("b");
