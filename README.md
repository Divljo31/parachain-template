<div align="center">

# 🚀 Parachain Template

A simple, ready-to-use parachain template that you can start with one command.

## 🎯 Quick Start

```bash
# Clone the repository
git clone <your-repo-url>
cd parachain-template

# Start your parachain with one command
./start.sh
```

That's it! Your parachain will be running and accessible at:
**https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944**

## 📋 Prerequisites

Before running the script, make sure you have:

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **polkadot-omni-node**: `cargo install --locked polkadot-omni-node@0.5.0`
- **chain-spec-builder**: `cargo install --locked chain-spec-builder@10.0.0`

## 🚀 What the Script Does

The `./start.sh` script automatically:

1. ✅ **Compiles the runtime** (if not already built)
2. ✅ **Generates chain specification** (if missing)
3. ✅ **Starts the parachain node**
4. ✅ **Shows connection info** for Polkadot.js Apps

## 🌐 Connecting to Your Parachain

Once the script is running, you can interact with your parachain using:

### Polkadot.js Apps (Recommended)
- **Direct link**: https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944
- **Manual connection**: Go to polkadot.js.org/apps → Development → Custom → `ws://localhost:9944`

### What You'll See
- **Chain name**: "parachain-template-runtime" in the top left
- **Block explorer**: Real-time block production
- **All parachain functionality**: Available through the UI

## 🛠️ Alternative Scripts

### Development Mode
For a more detailed development setup with metrics:

```bash
./scripts/dev-mode.sh
```

This script provides:
- Development block time settings
- Prometheus metrics at http://localhost:9615
- Enhanced RPC settings

## 🛑 Stopping the Parachain

To stop your parachain, simply press `Ctrl+C` in the terminal where the script is running.

## 📁 Project Structure

```
parachain-template/
├── start.sh              # Main startup script
├── scripts/
│   └── dev-mode.sh       # Development mode script
├── runtime/              # Parachain runtime logic
├── pallets/              # Custom pallets
└── node/                 # Node implementation
```

## 🆘 Troubleshooting

### Script fails to start
- Make sure all prerequisites are installed
- Check that you're in the correct directory
- Ensure you have sufficient disk space for compilation

### Can't connect to Polkadot.js Apps
- Verify the parachain is running (you should see block production)
- Check that port 9944 is not blocked by firewall
- Try refreshing the Polkadot.js Apps page

### Runtime compilation issues
- Update Rust: `rustup update`
- Clean and rebuild: `cargo clean && ./start.sh`

## 📚 Learn More

- [Polkadot Documentation](https://docs.polkadot.com/)
- [Substrate Documentation](https://docs.substrate.io/)
- [Polkadot.js Apps](https://polkadot.js.org/apps)
