use cosmwasm_std::{Addr, Empty};
use cw_access_contract_pkg::types::SaltItems;
use cw_storage_plus::{Item, Map};

pub const CREATOR: Item<Addr> = cw_access_contract_pkg::state::CREATOR;
pub const ALLOWED_CALLERS: Map<Addr, Empty> = cw_access_contract_pkg::state::ALLOWED_CALLERS;
pub const ALLOWED_METHODS: Map<String, Empty> = cw_access_contract_pkg::state::ALLOWED_METHODS;
pub const TARGET_CONTRACT: Item<Addr> = cw_access_contract_pkg::state::TARGET_CONTRACT;
pub const CRONCAT_FACTORY_CONTRACT: Item<Addr> =
    cw_access_contract_pkg::state::CRONCAT_FACTORY_CONTRACT;
pub const BIRTH_SALT: Item<SaltItems> = cw_access_contract_pkg::state::BIRTH_SALT;
