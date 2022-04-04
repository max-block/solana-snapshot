use clap::Parser;
use solana_snapshot_downloader::{run, Params};

/// Download snapshot  
#[derive(Parser)]
struct Cli {
    #[clap(short = 'd', default_value = "/tmp")]
    download_dir: String,

    #[clap(short = 'u', default_value = "https://api.mainnet-beta.solana.com")]
    rpc_url: String,
}

impl Cli {
    fn to_params(&self) -> Params {
        Params {
            download_dir: self.download_dir.clone(),
            rpc_url: self.rpc_url.clone(),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    run(cli.to_params()).unwrap();
}
