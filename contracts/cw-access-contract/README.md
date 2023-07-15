# Access Contract

## Overview

This is the base contract used when you want to create a CosmWasm contract that only you control. You grant it the ability to execute *any* methods on *any* contracts, but then uses contract logic in the `exec` method to ensure that only the methods you intend to be called will run authz execute. (Meaning, this contract behaves exactly like you at that moment.)

Please be aware of the amazing CosmWasm authz protobuf here:

https://github.com/CosmWasm/wasmd/blob/788cdbbfc3788b97799cf9c075cd4c4e5e4bbb8a/proto/cosmwasm/wasm/v1/authz.proto

This gives you access on a more granular level. This contract explores how to manually be more fine-grained, so we can rapidly innovate on account abstraction using CosmWasm and move tremendously faster.

## Build and optimize

    cd contracts/cw-access-contract
    cargo opt

## Store

    junod tx wasm store ../../artifacts/cw_access_contract.wasm --from mikereg -y

Take the code ID, throw it into the next command… you know the drill. (I prefer not to have a long jq command that grabs this, even though it can be done that way.)

## Instantiate

Here's the JSON in pretty form that we'll use:

```json
{
  "birth_salt": {
    "target_contract": "juno1rpaj83s6k9yaxetmzra4qcgr9xazelqvugy8wzdymeswppq0nlgsr0jx29",
    "allowed_methods": [
      "toggle"
    ],
    "delta": 0
  }
}
```

Here's the command, where you'll replace the code ID. Again, take the returned contract address and throw it into the following command…

    junod tx wasm instantiate 2873 '{"birth_salt":{"target_contract":"juno1rpaj83s6k9yaxetmzra4qcgr9xazelqvugy8wzdymeswppq0nlgsr0jx29","allowed_methods":["toggle"],"delta":0}}' --label "Access Contract" --no-admin --from mikereg -y

## Give authz permissions

This will grant the contract permissions to execute on your behalf. This is a "blanket" grant, and we're doing this instead of using a `ContractExecutionAuthorization`. You can see that the contract itself will "deconstruct" the `CosmosMsg` and essentially manually determine if it wants to end up running authz's `MsgExec` or not.

    junod tx authz grant juno1mcgq05alsz5q2746n0xej09urylf2cqvgyg40meauz9h4hf9m74sq03ynd generic --msg-type=/cosmwasm.wasm.v1.MsgExecuteContract --from mikereg -y | jq | head -n 42

## Check authz grants

For funsies:

    junod q authz grants-by-granter $(junod keys show mikereg -a) | jq

## Check the boolean value before

    junod q wasm contract-state smart juno1rpaj83s6k9yaxetmzra4qcgr9xazelqvugy8wzdymeswppq0nlgsr0jx29 '{"get_value":{}}'

## Call

Now we're calling this sucker, it determines that we're allowed to execute this method, and then does so.

    junod tx wasm execute juno1mcgq05alsz5q2746n0xej09urylf2cqvgyg40meauz9h4hf9m74sq03ynd '{"exec":{"cosmos_msg":{"wasm":{"execute":{"contract_addr":"juno1rpaj83s6k9yaxetmzra4qcgr9xazelqvugy8wzdymeswppq0nlgsr0jx29","funds":[],"msg":"eyJ0b2dnbGUiOnt9fQ=="}}}}}' --from mikereg -y

## Check the boolean value after

    junod q wasm contract-state smart juno1rpaj83s6k9yaxetmzra4qcgr9xazelqvugy8wzdymeswppq0nlgsr0jx29 '{"get_value":{}}'

------

You can see that we've called a smart contract that has executed on our behalf.