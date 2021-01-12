#!/bin/bash

near call $1.testnet deploy_subaccount1 '{"wrap": {"a": "some string"}, "sub": "2"}' --accountId $1.testnet
