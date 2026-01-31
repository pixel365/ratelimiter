use clap::{Parser, ValueEnum};
use std::net::IpAddr;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[arg(long = "max-key-length")]
    pub max_key_length: Option<usize>,

    #[arg(long = "protocol", value_enum)]
    pub protocol: Option<Vec<Protocol>>,

    #[arg(long = "http-host")]
    pub http_host: Option<IpAddr>,

    #[arg(long = "http-port")]
    pub http_port: Option<u16>,

    #[arg(long = "tcp-host")]
    pub tcp_host: Option<IpAddr>,

    #[arg(long = "tcp-port")]
    pub tcp_port: Option<u16>,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum Protocol {
    Http,
    Tcp,
    Grpc,
}
