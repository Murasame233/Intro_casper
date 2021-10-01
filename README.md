# Intro

A intro to Casper smartcontract dev.

# What it can do

Just a counter, start from 0.

# Run

Install the `Rust` and the `WebAssembly Binary Toolkit`.

```
make prepare
```

# the problem

> When on the first version dev.
> We use `add_contract_version` on the `call` to upgrade the smart contract.
> And the counter value put on global state is before upgrade.
> After update, The global state is been interrupted.
> So after update, we use `storage::write` or `storage::add` will have unreachable problem.
> And use context to query the counter, It will shows the value before upgrade.

Solve this problem.
When we create the counter before update,
we create a `URef` to this gloabal state and pass it on the `add_contract_version`'s parameter `named_keys`.
Then on the contract upgrade

> P.S.
> You can consider `URef` as pointer.
> You can consider the `call` function as a class's once `init`. We use this function to create other member function.(If you want upgrade contract twice, you can make a funtion like `upgrade`)
