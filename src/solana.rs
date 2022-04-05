use std::time::{Duration, Instant};

use solana_client::rpc_client::RpcClient;

use super::error::Error;

#[derive(Debug)]
pub struct Node {
    pub pubkey: String,
    pub rpc: String,
    pub snapshot_slot: u64,
    pub latency: u128,
}

impl Node {
    fn new(pubkey: String, rpc: String) -> Self {
        Node { pubkey, rpc, snapshot_slot: 0, latency: 0 }
    }
}

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
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    pool.scope(move |s| {
        for node in nodes.iter_mut() {
            let tx = tx.clone();
            s.spawn(move |_| {
                tx.send(update_snapshot_slot(node)).unwrap();
            });
        }
    });
    for _ in rx {}
}

fn update_snapshot_slot(node: &mut Node) {
    let client =
        RpcClient::new_with_timeout(&format!("http://{}", node.rpc), Duration::from_secs(3));
    let now = Instant::now();
    if let Ok(res) = client.get_highest_snapshot_slot() {
        node.latency = now.elapsed().as_millis();
        node.snapshot_slot = res.full;
    }
}
