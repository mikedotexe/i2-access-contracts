# Access Account Extruder (ACE)

## Overview

This contract uses Instantiate2 to create an account unique to the sender. You offer it a target contract, which methods it should be able to call, and it'll generate a deterministic salt.

The next step is to actually grant an authz ContractExecutionAuthorization. Then you, your keys, and other addresses (like CronCat) might call your account and trigger an execution on your behalf.

## General Extruder

    junod tx wasm store ../../artifacts/cw_generic_extruder.wasm --from mikereg -y

    junod tx wasm instantiate 2845 '{}' --label "Generic Extruder" --no-admin --from mikereg -y

    junod tx wasm execute juno1stxk2qdu22dyl034t680l85y23ydem8dmndtt4myxjff9jcx2jssalrmsc '{"create":{"code_id":2829}}' --from mikereg --gas-adjustment 1.7 --gas-prices 0.1ujunox -y

    junod q wasm contract juno17zl27ly9gjj97t343sm4uge3k2fqfcpa5hn9fxrdkrjy8r3rrexs6t9z2k

    junod tx wasm store ../../artifacts/cw_access_contract_extruder.wasm --from mikereg -y

    junod tx wasm instantiate 2876 '{"owner":"juno1yhqft6d2msmzpugdjtawsgdlwvgq3samrm5wrw"}' --label "Access Contract Extruder" --no-admin --from mikereg -y

JSON:

{
    "code_id": "2873",
    "target_contract": "juno1mcgq05alsz5q2746n0xej09urylf2cqvgyg40meauz9h4hf9m74sq03ynd",
    "allowed_methods": [
        "toggle"
    ],
    "delta": 0
}

    junod tx wasm execute juno1dc7ztf3jxsgatpku2mka66mkhfela8vfpexk8nthg9ajsgrj6hjqmz2s0z '{"create":{"code_id":"2873","target_contract":"juno1mcgq05alsz5q2746n0xej09urylf2cqvgyg40meauz9h4hf9m74sq03ynd","allowed_methods":["toggle"],"delta":0}}' --from mikereg -y

## 