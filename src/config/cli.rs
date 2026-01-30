use clap::{Parser, ValueEnum};
use std::net::IpAddr;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[arg(long)]
    pub max_key_length: Option<usize>,

    #[arg(long = "protocol", value_enum)]
    pub protocol: Option<Vec<Protocol>>,

    #[arg(long = "http-host")]
    pub http_host: Option<IpAddr>,

    #[arg(long)]
    pub http_port: Option<u16>,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum Protocol {
    Http,
    Tcp,
    Grpc,
}
