use std::time::Duration;

use solana_client::{client_error::ClientError, rpc_client::RpcClient};

pub struct Params {
    pub rpc_url: String,
    pub download_dir: String,
}

#[derive(Debug)]
pub enum Error {
    ClintError(String),
}

impl From<ClientError> for Error {
    fn from(e: ClientError) -> Self {
        Error::ClintError(e.to_string())
    }
}

#[derive(Debug)]
struct Node {
    pubkey: String,
    rpc: String,
    snapshot_slot: u64,
}

impl Node {
    fn update_snapshot_slot(&mut self) {
        println!("update_snapshot_slot: {}", self.rpc);
        let url = format!("http://{}", self.rpc);
        let client = RpcClient::new_with_timeout(&url, Duration::from_secs(3));
        if let Ok(res) = client.get_highest_snapshot_slot() {
            self.snapshot_slot = res.full;
        }
    }
}

pub fn run(params: Params) -> Result<(), Error> {
    let mut nodes = fetch_public_rpc_nodes(&params.rpc_url)?;
    dbg!(&nodes);
    update_snapshot_slot_for_nodes(&mut nodes);
    nodes = nodes
        .into_iter()
        .filter(|n| n.snapshot_slot > 0)
        .collect::<Vec<_>>();
    dbg!(&nodes);
    Ok(())
}

fn update_snapshot_slot_for_nodes(nodes: &mut Vec<Node>) {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(20)
        .build()
        .unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    pool.scope(move |s| {
        for node in nodes.iter_mut() {
            let tx = tx.clone();
            s.spawn(move |_| {
                tx.send(node.update_snapshot_slot()).unwrap();
            });
        }
    });
    for _ in rx {
        println!("done");
    }
}

fn fetch_public_rpc_nodes(url: &str) -> Result<Vec<Node>, Error> {
    let client = RpcClient::new_with_timeout(url, Duration::from_secs(10));
    let nodes = client.get_cluster_nodes()?;
    let nodes = nodes
        .into_iter()
        .filter(|n| n.rpc.is_some() && n.rpc.unwrap().is_ipv4())
        .map(|n| Node {
            pubkey: n.pubkey,
            rpc: n.rpc.unwrap().to_string(),
            snapshot_slot: 0,
        })
        .collect();
    Ok(nodes)
}
