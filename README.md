<div align="center">

# 🚀 Polkadot SDK Parachain Template

<img height="70px" alt="Polkadot SDK Logo" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_White.png#gh-dark-mode-only"/>
<img height="70px" alt="Polkadot SDK Logo" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only"/>

> A ready-to-use template for creating [parachains](https://wiki.polkadot.network/docs/learn-parachains) based on Polkadot SDK.
>
> **Quick Start**: Clone, run `./start.sh`, and connect to your local parachain! 🎯

</div>

</div>

## 🎯 Quick Start

```bash
# Clone the repository
git clone <your-repo-url>
cd parachain-template

# Start your parachain with one command
./start.sh
```

Then connect to your parachain at: **https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944**

---

## 📋 Table of Contents

- [Quick Start](#-quick-start)
- [Prerequisites](#prerequisites)
- [Template Structure](#template-structure)
- [Getting Started](#getting-started)
- [Starting a Development Chain](#starting-a-development-chain)
  - [One-Line Startup](#one-line-startup)
  - [Manual Setup](#manual-setup)
  - [Connect with Polkadot.js Apps](#connect-with-polkadotjs-apps)
- [Runtime Development](#runtime-development)
- [Contributing](#contributing)
- [Getting Help](#getting-help)

## 📋 Prerequisites

Before you start, make sure you have the following installed:

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **polkadot-omni-node**: `cargo install --locked polkadot-omni-node@0.5.0`
- **chain-spec-builder**: `cargo install --locked chain-spec-builder@10.0.0`

## 🏗️ Template Structure

This parachain template includes:

- ⏫ **Runtime** - Core logic of the parachain
- ☁️ **Cumulus Framework** - For parachain functionality
- 🔧 **Custom Pallets** - Starting point for your custom logic
- 💿 **Node** - Binary application (optional, uses Omni Node by default)
- 🚀 **One-Line Startup** - Simple `./start.sh` script

## 🎯 Intro

- ⏫ This template provides a starting point to build a [parachain](https://wiki.polkadot.network/docs/learn-parachains)
- ☁️ It is based on the [Cumulus](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/cumulus/index.html) framework
- 🔧 Its runtime is configured with a single custom pallet as a starting point, and ready-made pallets like [Balances](https://paritytech.github.io/polkadot-sdk/master/pallet_balances/index.html)
- 👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains)

## 📁 Template Structure

A Polkadot SDK based project such as this one consists of:

- 🧮 **[Runtime](./runtime/README.md)** - The core logic of the parachain
- 🎨 **[Pallets](./pallets/README.md)** - From which the runtime is constructed
- 💿 **[Node](./node/README.md)** - The binary application (optional, uses Omni Node by default)
- 🚀 **[Scripts](./scripts/)** - Helper scripts including the one-line startup

## 🚀 Getting Started

### Prerequisites

- 🦀 **Rust**: Check the [Rust installation instructions](https://www.rust-lang.org/tools/install) for your system
- 🛠️ **Dependencies**: Install required tools:
  ```bash
  cargo install --locked polkadot-omni-node@0.5.0
  cargo install --locked chain-spec-builder@10.0.0
  ```

### Clone the Repository

```bash
git clone <your-repo-url> parachain-template
cd parachain-template
```

## 🏃‍♂️ Starting a Development Chain

### 🚀 One-Line Startup (Recommended)

The easiest way to start your parachain:

```bash
./start.sh
```

This script will:
1. ✅ Check if runtime is built (builds if needed)
2. ✅ Generate chain specification (if missing)
3. ✅ Start the parachain node
4. ✅ Show connection info for Polkadot.js Apps

**Connect to your parachain**: https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944

### 📋 Manual Setup

The parachain template uses a hardcoded parachain ID defined in the runtime code:

```rust,ignore
pub const PARACHAIN_ID: u32 = 1000;
```

#### Build and Run Manually

If you prefer to run the steps manually:

1. **Build the runtime**:
   ```bash
   cargo build --release --locked
   ```

2. **Generate chain specification**:
   ```bash
   chain-spec-builder create -t development --relay-chain paseo --para-id 1000 --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm named-preset development
   ```

3. **Start the node**:
   ```bash
   polkadot-omni-node --chain ./chain_spec.json --dev
   ```

### Zombienet setup with Omni Node

Assuming we continue from the last step of the previous section, we have a chain spec and we need to setup a relay chain.
We can install `zombienet` as described [here](https://paritytech.github.io/zombienet/install.html#installation), and
`zombienet-omni-node.toml` contains the network specification we want to start.

#### Relay chain prerequisites

Download the `polkadot` (and the accompanying `polkadot-prepare-worker` and `polkadot-execute-worker`) binaries from
[Polkadot SDK releases](https://github.com/paritytech/polkadot-sdk/releases). Then expose them on `PATH` like so:

```sh
export PATH="$PATH:<path/to/binaries>"
```

#### Update `zombienet-omni-node.toml` with a valid chain spec path

To simplify the process of using the parachain-template with zombienet and Omni Node, we've added a pre-configured
development chain spec (dev_chain_spec.json) to the parachain template. The zombienet-omni-node.toml file of this
template points to it, but you can update it to an updated chain spec generated on your machine. To generate a
chain spec refer to [staging-chain-spec-builder](https://crates.io/crates/staging-chain-spec-builder)

Then make the changes in the network specification like so:

```toml
# ...
[[parachains]]
id = "<PARACHAIN_ID>"
chain_spec_path = "<TO BE UPDATED WITH A VALID PATH>"
# ...
```

#### Start the network

```sh
zombienet --provider native spawn zombienet-omni-node.toml
```

### Parachain Template Node

As mentioned in the `Template Structure` section, the `node` crate is optionally compiled and it is an alternative
to `Omni Node`. Similarly, it requires setting up a relay chain, and we'll use `zombienet` once more.

#### Install the `parachain-template-node`

```sh
cargo install --path node
```

#### Setup and start the network

For setup, please consider the instructions for `zombienet` installation [here](https://paritytech.github.io/zombienet/install.html#installation)
and [relay chain prerequisites](#relay-chain-prerequisites).

We're left just with starting the network:

```sh
zombienet --provider native spawn zombienet.toml
```

### 🌐 Connect with Polkadot.js Apps

Once your parachain is running, you can interact with it using Polkadot.js Apps:

#### Quick Connect
Click this link to connect directly: **https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944**

#### Manual Connection
1. Go to [polkadot.js.org/apps](https://polkadot.js.org/apps)
2. Click the network icon (top left corner)
3. Scroll to "Development" section
4. Select "Custom"
5. Enter: `ws://localhost:9944`
6. Click "Switch"

#### What You'll See
- **Chain name**: "parachain-template-runtime" in the top left
- **Block explorer**: Real-time block production
- **All parachain functionality**: Available through the UI

#### Alternative Hosts
- 🌐 **IPFS version**: [dotapps.io](https://dotapps.io/)
- 🧑‍🔧 **Self-hosted**: [polkadot-js/apps](https://github.com/polkadot-js/apps)

### Takeaways

Development parachains:

- 🔗 Connect to relay chains, and we showcased how to connect to a local one.
- 🧹 Do not persist the state.
- 💰 Are preconfigured with a genesis state that includes several prefunded development accounts.
- 🧑‍⚖️ Development accounts are used as validators, collators, and `sudo` accounts.

## Runtime development

We recommend using [`chopsticks`](https://github.com/AcalaNetwork/chopsticks) when the focus is more on the runtime
development and `OmniNode` is enough as is.

### Install chopsticks

To use `chopsticks`, please install the latest version according to the installation [guide](https://github.com/AcalaNetwork/chopsticks?tab=readme-ov-file#install).

### Build a raw chain spec

Build the `parachain-template-runtime` as mentioned before in this guide and use `chain-spec-builder`
again but this time by passing `--raw-storage` flag:

```sh
chain-spec-builder create --raw-storage --relay-chain "rococo-local" --para-id {{PARACHAIN_ID}} --runtime \
    target/release/wbuild/parachain-template-runtime/parachain_template_runtime.wasm named-preset development
```

### Start `chopsticks` with the chain spec

```sh
npx @acala-network/chopsticks@latest --chain-spec <path/to/chain_spec.json>
```

### Alternatives

`OmniNode` can be still used for runtime development if using the `--dev` flag, while `parachain-template-node` doesn't
support it at this moment. It can still be used to test a runtime in a full setup where it is started alongside a
relay chain network (see [Parachain Template node](#parachain-template-node) setup).

## Contributing

- 🔄 This template is automatically updated after releases in the main [Polkadot SDK monorepo](https://github.com/paritytech/polkadot-sdk).

- ➡️ Any pull requests should be directed to this [source](https://github.com/paritytech/polkadot-sdk/tree/master/templates/parachain).

- 😇 Please refer to the monorepo's
  [contribution guidelines](https://github.com/paritytech/polkadot-sdk/blob/master/docs/contributor/CONTRIBUTING.md) and
  [Code of Conduct](https://github.com/paritytech/polkadot-sdk/blob/master/docs/contributor/CODE_OF_CONDUCT.md).

## Getting Help

- 🧑‍🏫 To learn about Polkadot in general, [docs.Polkadot.com](https://docs.polkadot.com/) website is a good starting point.

- 🧑‍🔧 For technical introduction, [here](https://github.com/paritytech/polkadot-sdk#-documentation) are
  the Polkadot SDK documentation resources.

- 👥 Additionally, there are [GitHub issues](https://github.com/paritytech/polkadot-sdk/issues) and
  [Substrate StackExchange](https://substrate.stackexchange.com/).
- 👥You can also reach out on the [Official Polkdot discord server](https://polkadot-discord.w3f.tools/)
- 🧑Reach out on [Telegram](https://t.me/substratedevs) for more questions and discussions
