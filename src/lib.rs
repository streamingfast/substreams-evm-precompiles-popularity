mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::{pb::substreams::Clock, Hex};

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

use crate::pb::sf::substreams::ethereum::v1::Calls;

substreams_ethereum::init!();

fn map_precompile1_calls(clock: &Clock, calls_message: &Calls, output: &mut contract::Calls) {
    output
        .precompile1_call_runs
        .extend(&mut calls_message.calls.iter().filter_map(|call_info| {
            let call = call_info.call.as_ref().unwrap();

            match abi::precompile1_contract::functions::Run::decode(call) {
                Ok(decoded_call) => Some(contract::Precompile1RunCall {
                    call_tx_hash: call_info.tx_hash.clone(),
                    call_block_time: clock.timestamp.clone(),
                    call_block_number: clock.number,
                    call_ordinal: call.begin_ordinal,
                    call_success: !call.state_reverted,
                }),
                Err(_) => None,
            }
        }));
}

#[substreams::handlers::map]
fn map_calls(clock: Clock, calls: Calls) -> Result<contract::Calls, substreams::errors::Error> {
    let mut output = contract::Calls::default();
    map_precompile1_calls(&clock, &calls, &mut output);
    Ok(output)
}
