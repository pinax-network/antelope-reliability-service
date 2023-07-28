extern crate core;

mod pb;

use pb::antelope;
use prost::Message;
use chrono::*;

use crate::pb::pinax::service::v1::*;
use crate::pb::antelope::antelope_block_meta::v1::*;
use substreams::proto;
use substreams_sink_kv::prelude::*;

#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn pinax_service_v1_antelopereliability_countmissingblocks(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = CountMissingBlocksRequest::decode(&v[..]).expect("[BlockRangeRequest] Failed to decode");
    let store = Store::new();

    let kv_pairs = store.scan(format!("date: {}", req.start_date), format!("date: {}", req.end_date), Some(0 as u32));

    let mut missing_block_times: Vec<MissingBlockTime> = Vec::new();

    let mut missing_blocks = 0;
    for i in 0..kv_pairs.pairs.len() - 1 {
        let current_block = value_to_block(&kv_pairs.pairs[i].value).unwrap();
        let next_block = value_to_block(&kv_pairs.pairs[i + 1].value).unwrap();

        let mut count = count_half_second_differences(&current_block.timestamp, &next_block.timestamp);
        if count > 1 {
            count = count -1;
            //log::info!("Missing blocks: {}", count);
            missing_block_times.push(MissingBlockTime {
                number: count.to_string(),
                date_time: current_block.timestamp,
            });
        }
    }

    if missing_block_times.len() == 0 {
        missing_block_times.push(MissingBlockTime {
            number: "0".to_string(),
            date_time: "no date".to_string(),
        });
    }
    let mut out = MissingBlocks {
        missing_blocks: missing_block_times,
    };


    return Ok(out.encode_to_vec());
}

 
pub fn value_to_block(v: &Vec<u8>) -> Option<AntelopeBlockMeta> {
    let antelope_block_meta = AntelopeBlockMeta::decode(&v[..]).expect("failed to decode blockmeta");

    let mut blk = AntelopeBlockMeta {
        hash: antelope_block_meta.hash,
        producer: antelope_block_meta.producer,
        current_schedule: antelope_block_meta.current_schedule,
        timestamp: antelope_block_meta.timestamp,
    };

    return Some(blk);
}

fn count_half_second_differences(date_str1: &str, date_str2: &str) -> i32 {
    let date_time1 = Utc.datetime_from_str(date_str1, "%Y-%m-%dT%H:%M:%S%.3fZ").unwrap();
    let date_time2 = Utc.datetime_from_str(date_str2, "%Y-%m-%dT%H:%M:%S%.3fZ").unwrap();

    let duration = date_time2 - date_time1;
    let half_second_diff = Duration::milliseconds(500);

    (duration.num_milliseconds() / half_second_diff.num_milliseconds()) as i32
}