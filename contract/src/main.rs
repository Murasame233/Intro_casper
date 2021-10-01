#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::vec;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, ApiError, CLType, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Key, URef,
};

const KEY_NAME: &str = "count";

#[no_mangle]
pub extern "C" fn count_one() {
    let uref: URef = runtime::get_key(KEY_NAME)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::add(uref, 1);
}

#[no_mangle]
pub extern "C" fn call() {
    let vref = storage::new_uref(0);
    let key = Key::URef(vref);
    runtime::put_key(KEY_NAME, key);
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    let mut entrypoints = EntryPoints::new();
    entrypoints.add_entry_point(EntryPoint::new(
        "count_one",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let mut keys = NamedKeys::new();
    keys.insert(KEY_NAME.into(), key);
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entrypoints, keys);
    runtime::put_key("counter_contract", contract_hash.into());
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key("counter_contract_hash", contract_hash_pack.into())
}
