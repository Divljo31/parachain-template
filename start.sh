#!/bin/bash
# Parachain Template Starter - Tutorial Version
# Usage: ./start.sh

set -e

echo "🚀 Starting Parachain Template..."

# Step 1: Compile the runtime (if not already built)
if [ ! -f "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm" ]; then
    echo "🔨 Compiling the runtime..."
    cargo build --release --locked
fi

# Step 2: Generate the chain specification file (if not exists)
if [ ! -f "chain_spec.json" ]; then
    echo "📋 Generating chain specification file..."
    chain-spec-builder create -t development \
        --relay-chain paseo \
        --para-id 1000 \
        --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
        named-preset development
fi

# Step 3: Start the local chain
echo "🌟 Starting the local chain..."
echo "📡 Node accessible at: ws://localhost:9944"
echo "🌐 Polkadot.js Apps: https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944"
echo "⏹️  Press Ctrl+C to stop the node"
echo ""

# Start the omni node with CORS enabled for browser connections
polkadot-omni-node --chain ./chain_spec.json --dev --rpc-cors all
