use std::time::Duration;

use solana_client::rpc_client::RpcClient;

fn main() {
    let client = RpcClient::new_with_timeout("https://api.mainnet-beta.solana.com", Duration::from_secs(3));
    
    let res = client.get_highest_snapshot_slot().unwrap();
    dbg!(res);
}

