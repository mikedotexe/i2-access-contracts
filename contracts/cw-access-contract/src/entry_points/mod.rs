//! CosmWasm smart contracts contain entry points.
//! More on this pattern here: [https://book.cosmwasm.com/basics/entry-points.html](https://book.cosmwasm.com/basics/entry-points.html)

/// The execute entry point, to execute mutating methods
pub mod execute;
/// The instantiate entry point, called once
pub mod instantiate;
/// The migrate entry point to change code IDs
pub mod migrate;
/// The query entry point to get values
pub mod query;
