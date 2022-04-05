use std::path::Path;

use crate::download::{download_file, measure_download_speed};
use crate::model::Params;
use crate::output::{print_good_rpc_nodes_stats, print_params, print_public_rpc_nodes_stats};
use crate::solana::{fetch_snapshot_slots, get_current_slot, get_public_rpc_nodes};

use self::error::Error;

mod download;
mod error;
pub mod model;
mod output;
mod solana;

pub fn download_snapshot(params: Params) -> Result<(), Error> {
    print_params(&params);

    let mut nodes = get_public_rpc_nodes(&params.rpc_url)?;
    print_public_rpc_nodes_stats(&nodes, params.silent);

    let current_slot = get_current_slot(&params.rpc_url)?;
    fetch_snapshot_slots(&mut nodes, params.threads);
    nodes = nodes
        .into_iter()
        .filter(|n| current_slot - n.snapshot_slot <= params.max_slot_distance && !n.snapshot_path.ends_with(".tar"))
        .collect::<Vec<_>>();
    if nodes.len() == 0 {
        return Err(Error::NoGoodNodes);
    }
    nodes.sort_by_key(|n| n.latency);
    print_good_rpc_nodes_stats(&nodes, params.silent);

    let mut count = 0;
    for node in nodes.iter_mut() {
        measure_download_speed(node, params.measure_time);
        count += 1;
        if count > params.measure_count {
            break;
        }
        dbg!(node);
    }

    nodes.sort_by_key(|n| n.measure_download);
    nodes.reverse();
    dbg!(&nodes);
    let node = &nodes[0];

    download_file(
        &format!("http://{}/snapshot.tar.bz2", node.rpc),
        Path::new(&params.download_dir).join(&node.snapshot_path).to_str().unwrap(),
    )?;
    dbg!(current_slot);
    Ok(())
}
