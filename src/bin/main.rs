use clap::Parser;

use solana_snapshot_downloader::{download_snapshot, Params};

/// Download snapshot
#[derive(Parser)]
struct Cli {
    #[clap(short = 'd', default_value = "/tmp")]
    download_dir: String,

    #[clap(short = 'u', default_value = "https://api.mainnet-beta.solana.com")]
    rpc_url: String,

    #[clap(short = 't', default_value = "30")]
    threads: usize,

    #[clap(short = 's', default_value = "1000")]
    max_slot_distance: u64,

    #[clap(long)]
    silent: bool,
}

impl Cli {
    fn to_params(&self) -> Params {
        Params {
            download_dir: self.download_dir.clone(),
            rpc_url: self.rpc_url.clone(),
            threads: self.threads,
            max_slot_distance: self.max_slot_distance,
            silent: self.silent,
        }
    }
}

fn main() {
    let cli = Cli::parse();
    download_snapshot(cli.to_params()).unwrap();
}
