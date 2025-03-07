mod pb;
use pb::contract::v1 as contract;
use substreams::{
    hex,
    store::{StoreAdd, StoreAddInt64, StoreGet, StoreGetInt64, StoreNew},
    Hex,
};

use crate::pb::sf::substreams::ethereum::v1::Calls;

const ECRECOVER: [u8; 20] = hex!("0000000000000000000000000000000000000001");
const SHA256HASH: [u8; 20] = hex!("0000000000000000000000000000000000000002");
const RIPEMD160HASH: [u8; 20] = hex!("0000000000000000000000000000000000000003");
const DATA_COPY: [u8; 20] = hex!("0000000000000000000000000000000000000004");
const BIG_MOD_EXP: [u8; 20] = hex!("0000000000000000000000000000000000000005");
const BIG_MOD_EXP_EIP2565: [u8; 20] = hex!("00000000000000000000000000000000000000f5");
const BN256_ADD_ISTANBUL: [u8; 20] = hex!("0000000000000000000000000000000000000006");
const BN256_SCALAR_MUL_ISTANBUL: [u8; 20] = hex!("0000000000000000000000000000000000000007");
const BN256_PAIRING_ISTANBUL: [u8; 20] = hex!("0000000000000000000000000000000000000008");
const BLAKE2_F: [u8; 20] = hex!("0000000000000000000000000000000000000009");
const KZG_POINT_EVALUATION: [u8; 20] = hex!("000000000000000000000000000000000000000a");
const BLS12381_G1_ADD: [u8; 20] = hex!("0000000000000000000000000000000000000f0a");
const BLS12381_G1_MULTI_EXP: [u8; 20] = hex!("0000000000000000000000000000000000000f0b");
const BLS12381_G2_ADD: [u8; 20] = hex!("0000000000000000000000000000000000000f0c");
const BLS12381_G2_MULTI_EXP: [u8; 20] = hex!("0000000000000000000000000000000000000f0d");
const BLS12381_PAIRING: [u8; 20] = hex!("0000000000000000000000000000000000000f0e");
const BLS12381_MAP_G1: [u8; 20] = hex!("0000000000000000000000000000000000000f0f");
const BLS12381_MAP_G2: [u8; 20] = hex!("0000000000000000000000000000000000000f10");

const PRECOMPILES: [[u8; 20]; 18] = [
    ECRECOVER,
    SHA256HASH,
    RIPEMD160HASH,
    DATA_COPY,
    BIG_MOD_EXP,
    BIG_MOD_EXP_EIP2565,
    BN256_ADD_ISTANBUL,
    BN256_SCALAR_MUL_ISTANBUL,
    BN256_PAIRING_ISTANBUL,
    BLAKE2_F,
    KZG_POINT_EVALUATION,
    BLS12381_G1_ADD,
    BLS12381_G1_MULTI_EXP,
    BLS12381_G2_ADD,
    BLS12381_G2_MULTI_EXP,
    BLS12381_PAIRING,
    BLS12381_MAP_G1,
    BLS12381_MAP_G2,
];

#[substreams::handlers::store]
fn store_counts(calls_message: Calls, s: StoreAddInt64) {
    for call_info in calls_message.calls {
        let call = call_info.call.unwrap();

        s.add(call.end_ordinal, Hex::encode(call.address), 1);
    }
}

#[substreams::handlers::map]
fn map_counts(
    counts: StoreGetInt64,
    _calls_message: Calls,
) -> Result<contract::PrecompilesPopularity, substreams::errors::Error> {
    let mut output = contract::PrecompilesPopularity::default();

    for precompile in PRECOMPILES {
        let key = Hex::encode(precompile);

        if let Some(count) = counts.get_last(&key) {
            output.entries.push(contract::Entry {
                name: precompile_to_name(precompile).to_string(),
                address: key,
                count,
            });
        }
    }
    Ok(output)
}

fn precompile_to_name(address: [u8; 20]) -> &'static str {
    match address {
        ECRECOVER => "ecrecover",
        SHA256HASH => "sha256hash",
        RIPEMD160HASH => "ripemd160hash",
        DATA_COPY => "datacopy",
        BIG_MOD_EXP => "bigmodexp",
        BIG_MOD_EXP_EIP2565 => "bigmodexp (eip2565)",
        BN256_ADD_ISTANBUL => "bn256add",
        BN256_SCALAR_MUL_ISTANBUL => "bn256scalarmul",
        BN256_PAIRING_ISTANBUL => "bn256pairing",
        BLAKE2_F => "blake2f",
        KZG_POINT_EVALUATION => "kzgpointevaluation",
        BLS12381_G1_ADD => "bls12381g1add",
        BLS12381_G1_MULTI_EXP => "bls12381g1multiexp",
        BLS12381_G2_ADD => "bls12381g2add",
        BLS12381_G2_MULTI_EXP => "bls12381g2multiexp",
        BLS12381_PAIRING => "bls12381pairing",
        BLS12381_MAP_G1 => "bls12381mapg1",
        BLS12381_MAP_G2 => "bls12381mapg2",
        _ => "unknown",
    }
}
