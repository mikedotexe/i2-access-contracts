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

    neutrond tx wasm store ../../artifacts/cw_access_contract.wasm --from mike --gas auto --gas-prices 0.025untrn --gas-adjustment 1.3 -y

### Mainnet

- code ID: `100`

Take the code ID, throw it into the next command… you know the drill. (I prefer not to have a long jq command that grabs this, even though it can be done that way.)

## Instantiate

Here's the JSON in pretty form that we'll use:

```json
{
  "birth_salt": {
    "target_contract": "neutron10memhlzs9aqtkm8c36j3ufuquv96nlk3drc7n4rgyrfa8ke67aysh2fd0j",
    "allowed_methods": [
      "toggle"
    ],
    "delta": 0
  },
  "allowed_callers": [
    "neutron1qv6mptxcz8zsjdze0mcfwrk6kx33pqwkf9nqsm"
  ]
}
```

Here's the command, where you'll replace the code ID. Again, take the returned contract address and throw it into the following command…

    neutrond tx wasm instantiate 100 '{"birth_salt":{"target_contract":"neutron10memhlzs9aqtkm8c36j3ufuquv96nlk3drc7n4rgyrfa8ke67aysh2fd0j","allowed_methods":["toggle"],"delta":0},"allowed_callers":["neutron1qv6mptxcz8zsjdze0mcfwrk6kx33pqwkf9nqsm"]}' --label "Access Contract for Boolean Contract" --no-admin --from mike --from mike --gas auto --gas-prices 0.025untrn --gas-adjustment 1.3 -y

### Mainnet

- contract: `neutron1wgps0yxma32a0cdkqx62ujnsprn5qw9farjr9wg9yys4h2sa0rzq96mylv`

## Give authz permissions

This will grant the contract permissions to execute on your behalf. This is a "blanket" grant, and we're doing this instead of using a `ContractExecutionAuthorization`. You can see that the contract itself will "deconstruct" the `CosmosMsg` and essentially manually determine if it wants to end up running authz's `MsgExec` or not.

    neutrond tx authz grant neutron1wgps0yxma32a0cdkqx62ujnsprn5qw9farjr9wg9yys4h2sa0rzq96mylv generic --msg-type=/cosmwasm.wasm.v1.MsgExecuteContract --from mike --gas auto --gas-prices 0.025untrn --gas-adjustment 1.3 -y | jq | head -n 42

## Check authz grants

For funsies:

    neutrond q authz grants-by-granter $(neutrond keys show mike -a) | jq

## Check the boolean value before

    neutrond q wasm contract-state smart neutron10memhlzs9aqtkm8c36j3ufuquv96nlk3drc7n4rgyrfa8ke67aysh2fd0j '{"get_value":{}}'

## Call

Now we're calling this sucker, it determines that we're allowed to execute this method, and then does so.

    neutrond tx wasm execute neutron1wgps0yxma32a0cdkqx62ujnsprn5qw9farjr9wg9yys4h2sa0rzq96mylv '{"exec":{"cosmos_msg":{"wasm":{"execute":{"contract_addr":"neutron10memhlzs9aqtkm8c36j3ufuquv96nlk3drc7n4rgyrfa8ke67aysh2fd0j","funds":[],"msg":"eyJ0b2dnbGUiOnt9fQ=="}}}}}' --from hackwasm2023 --gas auto --gas-prices 0.025untrn --gas-adjustment 1.3 -y

## Check the boolean value after

    neutrond q wasm contract-state smart neutron10memhlzs9aqtkm8c36j3ufuquv96nlk3drc7n4rgyrfa8ke67aysh2fd0j '{"get_value":{}}'

------

You can see that we've called a smart contract that has executed on our behalf.