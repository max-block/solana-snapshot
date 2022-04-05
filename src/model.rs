#[derive(Debug)]
pub struct Params {
    pub rpc_url: String,
    pub download_dir: String,
    pub threads: usize,
    pub max_slot_distance: u64,
    pub silent: bool,
    pub measure_time: u64,
    pub measure_count: u8,
}

#[derive(Debug)]
pub struct Node {
    pub pubkey: String,
    pub rpc: String,
    pub snapshot_slot: u64,
    pub latency: u128,
    pub snapshot_path: String,
    pub measure_download: u64,
}

impl Node {
    pub fn new(pubkey: String, rpc: String) -> Self {
        Node { pubkey, rpc, snapshot_slot: 0, latency: 0, snapshot_path: String::new(), measure_download: 0 }
    }
}
