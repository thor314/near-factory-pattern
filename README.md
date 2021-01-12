# Demonstration Goals:
- Write a deploy method for a contract that accepts a custom `struct` as argument for its constructor
- Write a contract that demonstrates the factory pattern, deploying other contracts to subaddresses

# Reproduce:
1. visit https://wallet.testnet.near.org/profile and create a new test account, YOU
2. `near login`
3. `./build.sh`
4. `./deploy.sh YOU`
5. `./sub_deploy.sh YOU`
Note that deploying the sub_accounts requires more than the default amount of gas.

Optionally:
`./delete_accounts.sh YOU` to delete subaccount 1.

current error on call:
`near call $1.testnet deploy_subaccount1 '{"wrap": {"a": "some string"}, "sub": "1"}' --accountId $1.testnet
--amount 50 --gas 150000000000000`

`Failure [mb12.testnet]: Error: The account 1.mb12.testnet wouldn't have enough balance to cover storage, required to have 106084300000000000000000000`
