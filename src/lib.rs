mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const PRECOMPILE1_TRACKED_CONTRACT: [u8; 20] = hex!("0000000000000000000000000000000000000001");

fn map_precompile1_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.precompile1_call_runs.append(
        &mut blk
            .transactions()
            .flat_map(|tx| {
                tx.calls
                    .iter()
                    .filter(|call| {
                        call.address == PRECOMPILE1_TRACKED_CONTRACT
                            && abi::precompile1_contract::functions::Run::match_call(call)
                    })
                    .filter_map(|call| {
                        match abi::precompile1_contract::functions::Run::decode(call) {
                            Ok(decoded_call) => Some(contract::Precompile1RunCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            }),
                            Err(_) => None,
                        }
                    })
            })
            .collect(),
    );
}

#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
    let mut calls = contract::Calls::default();
    map_precompile1_calls(&blk, &mut calls);
    Ok(calls)
}
