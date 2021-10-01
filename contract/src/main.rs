#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use core::convert::TryInto;

use alloc::vec;

use casper_contract::contract_api::{runtime, storage};
use casper_types::{ApiError, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key,CLType};

const KEY_NAME: &str = "count";

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    KeyAlreadyExists = 0,
    KeyMismatch = 1,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn count_one(){
    let missing_key = runtime::get_key(KEY_NAME);
    if missing_key.is_none() {
        let vref = storage::new_uref(0);
        let key = Key::URef(vref);
        runtime::put_key(KEY_NAME, key);
    }
    let key = runtime::get_key(KEY_NAME).unwrap().into_uref().unwrap();
    storage::add(key,1);
}

fn check(){
    let missing_key = runtime::get_key(KEY_NAME);
    if missing_key.is_none() {
        let vref = storage::new_uref(0);
        let key = Key::URef(vref);
        runtime::put_key(KEY_NAME, key);
    }
}

// #[no_mangle]
// pub extern "C" fn get_count(){
//     let key = runtime::get_key(KEY_NAME).unwrap().try_into().unwrap();
//     runtime::ret(storage::read(key).unwrap().unwrap());
// }

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    let mut entrypoints = EntryPoints::new();
    check();
    // entrypoints.add_entry_point(EntryPoint::new(
    //     "get_count",
    //     vec![],
    //     casper_types::CLType::I32,
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract,
    // ));
    entrypoints.add_entry_point(EntryPoint::new(
        "count_one",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entrypoints, Default::default());
    runtime::put_key("counter_contract", contract_hash.into());
    let contract_hash_pack = storage::new_uref(contract_hash);
    runtime::put_key("counter_contract_hash", contract_hash_pack.into())
}
