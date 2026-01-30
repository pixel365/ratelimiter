use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[arg(long)]
    pub max_key_length: Option<usize>,

    #[arg(long = "protocol", value_enum)]
    pub protocol: Option<Vec<Protocol>>,

    #[arg(long)]
    pub http_port: Option<u16>,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum Protocol {
    Http,
    Tcp,
    Grpc,
}
