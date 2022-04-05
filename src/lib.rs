use std::path::Path;

use crate::download::download_file;
use crate::solana::{fetch_snapshot_slots, get_current_slot, get_public_rpc_nodes};

use self::error::Error;

mod download;
mod error;
mod solana;

pub struct Params {
    pub rpc_url: String,
    pub download_dir: String,
    pub threads: usize,
    pub max_slot_distance: u64,
    pub silent: bool,
}

pub fn download_snapshot(params: Params) -> Result<(), Error> {
    let mut nodes = get_public_rpc_nodes(&params.rpc_url)?;
    let current_slot = get_current_slot(&params.rpc_url)?;
    fetch_snapshot_slots(&mut nodes, params.threads);
    nodes = nodes
        .into_iter()
        .filter(|n| current_slot - n.snapshot_slot <= params.max_slot_distance)
        .collect::<Vec<_>>();
    if nodes.len() == 0 {
        return Err(Error::NoGoodNodes);
    }
    nodes.sort_by_key(|n| n.latency);
    download_file(
        &format!("http://{}/snapshot.tar.bz2", nodes[0].rpc),
        Path::new(&params.download_dir).join("snapshot.tar.bz2").to_str().unwrap(),
    )?;
    dbg!(&nodes);
    dbg!(current_slot);
    Ok(())
}
