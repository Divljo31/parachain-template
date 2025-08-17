#!/bin/bash

# Simple Development Mode Script (No Zombienet Required)
# This script starts your parachain in standalone development mode

set -e

echo "🚀 Starting Parachain in Development Mode"
echo "=========================================="

# Check if chain_spec.json exists
if [ ! -f "chain_spec.json" ]; then
    echo "❌ chain_spec.json not found."
    echo "💡 To generate it, run:"
    echo "   chain-spec-builder create -t development --relay-chain paseo --para-id 1000 --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm named-preset development"
    exit 1
fi

echo "📡 Starting Omni Node in development mode..."
echo "🌐 Your parachain will be available at:"
echo "   https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944"
echo "📊 Metrics available at: http://localhost:9615"
echo "⏹️  Press Ctrl+C to stop the parachain"
echo ""

# Start the parachain with development settings
polkadot-omni-node \
    --chain chain_spec.json \
    --dev \
    --dev-block-time 1000 \
    --rpc-external \
    --rpc-cors all \
    --ws-external \
    --prometheus-external
