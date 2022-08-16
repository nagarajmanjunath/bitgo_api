use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Config {
    #[structopt(
        long = "bitgo-endpoint",
        env = "BITGO_ENDPOINT",
        about = "BitGo REST API endpoint"
    )]
    pub endpoint: String,
    #[structopt(
        long = "bitgo-token",
        env = "BITGO_TOKEN",
        about = "API key for the BitGo service"
    )]
    pub token: String,

    #[structopt(
        long = "bitgo-certpath",
        env = "BITGO_CERTPATH",
        about = "SSL public cert, path certificate for the BitGo service"
    )]
    pub bitgo_cert_path: Option<String>,
}
