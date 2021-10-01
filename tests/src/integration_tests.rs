#[cfg(test)]
mod tests {
    use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContextBuilder};
    use casper_types::{
        account::AccountHash, runtime_args, PublicKey, RuntimeArgs, SecretKey, U512,
    };

    const MY_ACCOUNT: [u8; 32] = [7u8; 32];
    const CONTRACT_WASM: &str = "contract.wasm";

    #[test]
    fn should_store_hello_world() {
        // Ready account
        let secret_key = SecretKey::ed25519_from_bytes(MY_ACCOUNT).unwrap();
        let public_key = PublicKey::from(&secret_key);
        let account_addr = AccountHash::from(&public_key);
        let mut context = TestContextBuilder::new()
            .with_public_key(public_key, U512::from(500_000_000_000_000_000u64))
            .build();

        // Deploy
        let session_code = Code::from(CONTRACT_WASM);
        let deploysession = SessionBuilder::new(session_code, runtime_args! {})
            .with_address(account_addr)
            .with_authorization_keys(&[account_addr])
            .build();
        context.run(deploysession);
        println!("Deployed");

        // Add count
        let hash: Hash = context
            .query(account_addr, &[String::from("counter_contract_hash")])
            .unwrap()
            .into_t()
            .unwrap();
        let code = Code::Hash(hash, "count_one".to_string());
        let addsession = SessionBuilder::new(code, runtime_args! {})
            .with_address(account_addr)
            .with_authorization_keys(&[account_addr])
            .build();
        context.run(addsession);

        // Query count
        let n: i32 = context
            .query(account_addr, &["count".into()])
            .unwrap()
            .into_t()
            .unwrap();
        println!("added");

        println!("n:{}", n);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
