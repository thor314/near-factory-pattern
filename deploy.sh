#!/bin/bash

near deploy --wasmFile res/struct_init.wasm --initFunction "new" --initArgs '{"wrap": {"a": "some string"}}' --accountId $1.testnet
