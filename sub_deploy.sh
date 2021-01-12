#!/bin/bash

near call $1.testnet deploy_subaccount1 '{"wrap": {"a": "some string"}, "sub": "1"}' --accountId $1.testnet --amount 50 --gas 150000000000000

# current error: 	Failure [mb12.testnet]: Error: The account 1.mb12.testnet wouldn't have enough balance to cover storage, required to have 106084300000000000000000000
