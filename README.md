# Instantiate2 Access Contracts

This repo is a set on contracts and their packages that help create a system where "access contracts" can be created and controlled by a user.

An access contract is one that is granted authz ContractExecutionAuthorization to execute given methods on a target contract, and execute as if it were the end user.

## Concept

At the time of this writing, there's 17 minutes until it's "pencils down" at the HackWasm 2023 in lovely Berlin.

There are three contracts here, so let's cover these quickly.

1. **Generic Extruder**: this contract takes some ideas from [Simon's blog here](https://medium.com/cosmwasm/dev-note-3-limitations-of-instantiate2-and-how-to-deal-with-them-a3f946874230) about the limitations of Instantiate2, and addresses this by essentially having "fixed values" so we can get the certainty we desire.
2. **ACE**: the "Access Contract Extruder" is similar, except it's specifically meant to be a factory that generates "access contracts" that will interact with, say, a DEX on your behalf. This contract should be granted a authz generic authorization for the message type `/cosmos.authz.v1beta1.MsgGrant`. This gives it the ability to grant authz privileges on your behalf to OTHER contracts; namely the contracts it creates using instantiate2.
3. **Access Contract**: this contract is able to call a specific smart contract at specific methods on behalf of an end user. The ACE contract ensures that the contract address also reflects the privileges this access contract is allowed to do. By default, the end user who controls this is allowed to call it, as well as any other addresses they've added as "allowed callers." Take a peek in the code to see how you an allowed caller can send a `CosmosMsg` and essentially deconstruct it into JSON such that it can control whether it wants to eventually use the "authz execute" mechanism to execute for the end user. Following this pattern, you can have bespoke rules for authz execution that transcend limitations on the protobuf level.

alright gotta go, 8 minutes til submission time
honey, I love you. mochi, I miss you little doggy
