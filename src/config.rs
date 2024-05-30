use std::net::SocketAddrV4;

use clap::Parser;
use url::Url;

#[derive(Parser, Clone)]
pub struct Config {
    #[clap(long, env)]
    pub url: Url,
    #[clap(long, env)]
    pub kafka_url: String,
    #[clap(long, env)]
    pub listen_on: SocketAddrV4,
    #[clap(long, env)]
    pub api_key: String,
    #[clap(long, env)]
    pub database_url: String,
}
