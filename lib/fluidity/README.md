# Fluidity

## Overview

Fluidity was created out of a general frustration with the current state of centralized and decentralized identity
solutions. As we progress towards the future it remains paramount that users become hyper aware of proper data
practices.

In order to best serve users Proton will automatically generate a personal constant size, EVM compatible blockchain
enabling them to store all of their personal information on. This benefits the sister chains, Aether and Chaos in a
number of ways as Flow serves the ecosystem the user's perspective enabling the independent networks to uniquely
converge, visually manifesting within Proton.

### Goals

* Develop tools enabling users to seamlessly transition onto the platform
* Create a constant size blockchain, similar to Mina, which is composed of other constant sized chains and validates the
  existence of the owner of the subspace

## Getting Started

    git clone https://github.com/scattered-systems/flow
    
    cargo build --release --workspace
    cargo test --all-features --color always --release --verbose --workspace

    cargo run --bin flow-api --release
    cargo run --bin flow-cli --release -- --help