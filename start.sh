#!/bin/bash
# One-line parachain starter
# Usage: ./start.sh

set -e

echo "🚀 Starting Parachain Template..."

# Check if chain_spec.json exists, if not generate it
if [ ! -f "chain_spec.json" ]; then
    echo "📋 Generating chain specification..."
    chain-spec-builder create -t development --relay-chain paseo --para-id 1000 --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm named-preset development
fi

# Check if runtime is built
if [ ! -f "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm" ]; then
    echo "🔨 Building runtime..."
    cargo build --release --locked
fi

echo "🌐 Starting parachain node..."
echo "📡 RPC: ws://localhost:9944"
echo "🌍 Polkadot.js: https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944"
echo "⏹️  Press Ctrl+C to stop"
echo ""

polkadot-omni-node --chain ./chain_spec.json --dev
