#!/bin/sh

./build.sh

echo ">> Deploying contract"

near deploy zvyazok.testnet --wasmFile ./target/wasm32-unknown-unknown/release/chat_contract.wasm