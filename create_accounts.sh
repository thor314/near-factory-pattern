#!/bin/bash

near create-account 1.$1.testnet --initialBalance 45 --masterAccount $1.testnet
near create-account 2.$1.testnet --initialBalance 45 --masterAccount $1.testnet
