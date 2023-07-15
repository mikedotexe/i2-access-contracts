//! Messages are how interactions occur in CosmWasm contracts.

/// Available execute messages (likely changing state)
pub mod execute_msg;
/// Available instantiate messages (the parameters sent to the bytecode when instantiating)
pub mod instantiate_msg;
/// Migration, which is used to overwrite this contract
pub mod migrate_msg;
/// Available query messages (read-only, not changing state)
pub mod query_msg;
