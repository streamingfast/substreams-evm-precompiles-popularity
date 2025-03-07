mod pb;
use pb::contract::v1 as contract;
use substreams::{
    store::{DeltaInt64, Deltas, StoreAdd, StoreAddInt64, StoreGet, StoreGetInt64, StoreNew},
    Hex,
};

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

use crate::pb::sf::substreams::ethereum::v1::Calls;

substreams_ethereum::init!();

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
) -> Result<contract::PrecompilesPopularity, substreams::errors::Error> {
    let mut output = contract::PrecompilesPopularity::default();

    for precompile in 1..12 {
        let address: [u8; 20] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, precompile,
        ];
        let key = Hex::encode(address);

        if let Some(count) = counts.get_last(&key) {
            output.entries.push(contract::Entry {
                address: key,
                count,
            });
        }
    }
    Ok(output)
}
