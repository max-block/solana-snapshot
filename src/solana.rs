use std::time::{Duration, Instant};

use reqwest::blocking::Client;
use solana_client::rpc_client::RpcClient;

use crate::model::Node;

use super::error::Error;

pub fn get_public_rpc_nodes(url: &str) -> Result<Vec<Node>, Error> {
    let client = RpcClient::new_with_timeout(url, Duration::from_secs(10));
    let nodes = client.get_cluster_nodes()?;
    let nodes = nodes
        .into_iter()
        .filter(|n| n.rpc.is_some() && n.rpc.unwrap().is_ipv4())
        .map(|n| Node::new(n.pubkey, n.rpc.unwrap().to_string()))
        .collect();
    Ok(nodes)
}

pub fn get_current_slot(url: &str) -> Result<u64, Error> {
    let client = RpcClient::new_with_timeout(url, Duration::from_secs(10));
    Ok(client.get_slot()?)
}

pub fn fetch_snapshot_slots(nodes: &mut Vec<Node>, threads: usize) {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    pool.scope(move |s| {
        for node in nodes.iter_mut() {
            let tx = tx.clone();
            s.spawn(move |_| {
                tx.send(update_snapshot_slot_info(node)).unwrap();
            });
        }
    });
    for _ in rx {}
}

fn update_snapshot_slot_info(node: &mut Node) {
    let client = Client::builder().timeout(Duration::from_secs(3)).build().unwrap();
    let url = format!("http://{}/snapshot.tar.bz2", node.rpc);
    let now = Instant::now();
    if let Ok(res) = client.head(url).send() {
        if res.status() == 200 {
            node.latency = now.elapsed().as_millis();
            node.snapshot_path = res.url().path().trim_start_matches("/").to_string();
            node.snapshot_slot = get_slot_from_snapshot_path(&node.snapshot_path).unwrap_or(0)
        }
    }
}

fn get_slot_from_snapshot_path(path: &str) -> Option<u64> {
    let parts: Vec<&str> = path.split("-").collect();
    if parts.len() == 3 {
        return parts[1].parse::<u64>().ok();
    }
    None
}
