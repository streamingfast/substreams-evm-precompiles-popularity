# Substreams EVM Precompiles Popularity

This project leverages Substreams to calculate the popularity of Ethereum precompiled contracts from call data.

## Overview

- Processes Ethereum call events using Substreams.
- Counts and maps occurrences of precompiles.
- Outputs results using Protocol Buffers.

## Getting Started

```bash
substreams gui substreams-evm-precompiles-popularity@v0.2.0
```

## Results

### Ethereum Mainnet

At block `#21 996 000`:

1. `0x0000000000000000000000000000000000000001` => 247543455 (`ecrecover`)
1. `0x0000000000000000000000000000000000000002` => 128524392 (`sha256Hash`)
1. `0x0000000000000000000000000000000000000004` => 46295370 (`dataCopy`)
1. `0x0000000000000000000000000000000000000006` => 44480667 (`bn256Add`)
1. `0x0000000000000000000000000000000000000007` => 44330095 (`bn256ScalarMul`)
1. `0x0000000000000000000000000000000000000005` => 16939742 (`bigModExp`)
1. `0x0000000000000000000000000000000000000008` => 2504972 (`bn256Pairing`)
1. `0x0000000000000000000000000000000000000003` => 618211 (`ripemd160Hash`)
1. `0x000000000000000000000000000000000000000a` => 570252 (`kzgPointEvaluation`)
1. `0x0000000000000000000000000000000000000009` => 161695 (`blake2F`)

### Compute Results

```bash
substreams run substreams-evm-precompiles-popularity@v0.2.0 map_counts -s 21996000 -t +1 -o jsonl | skip 1 | head -n1 | jq -rc '."@data".entries | sort_by(-(.count|tonumber)) | .[] | "0x\(.address) => \(.count) (\(.name))"'
```

## License

Apache 2.0
