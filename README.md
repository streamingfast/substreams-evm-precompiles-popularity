# Substreams EVM Precompiles Popularity

This project leverages Substreams to calculate the popularity of Ethereum precompiled contracts from call data.

## Overview

- Processes Ethereum call events using Substreams.
- Counts and maps occurrences of precompiles.
- Outputs results using Protocol Buffers.

## Getting Started

1. Ensure the Rust toolchain is installed.
2. Build the project: `cargo build --release`.
3. Deploy the WASM binary as specified in the Substreams configuration.

## License

Apache 2.0
