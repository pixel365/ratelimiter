use crate::config::cli::{Cli, Protocol};
use crate::core::defaults::{DEFAULT_HTTP_PORT, DEFAULT_MAX_KEY_LENGTH, DEFAULT_TCP_PORT};
use clap::Parser;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone, Default)]
pub struct RuntimeConfig {
    pub max_key_length: Option<u32>,
    pub protocol: Option<Vec<Protocol>>,
    pub http_host: Option<IpAddr>,
    pub http_port: Option<u16>,
    pub tcp_host: Option<IpAddr>,
    pub tcp_port: Option<u16>,
}

impl RuntimeConfig {
    pub fn new() -> Self {
        let file = RuntimeConfig::default();
        let env = RuntimeConfig::default();
        let cli = Cli::parse().into();

        file.merge(env).merge(cli).finalize()
    }

    fn merge(mut self, config: RuntimeConfig) -> RuntimeConfig {
        if config.max_key_length.is_some() {
            self.max_key_length = config.max_key_length;
        }

        if config.http_host.is_some() {
            self.http_host = config.http_host;
        }

        if config.http_port.is_some() {
            self.http_port = config.http_port;
        }

        if config.tcp_host.is_some() {
            self.tcp_host = config.tcp_host;
        }

        if config.tcp_port.is_some() {
            self.tcp_port = config.tcp_port;
        }

        if config.protocol.is_some() {
            self.protocol = config.protocol;
        }

        self
    }

    fn finalize(mut self) -> RuntimeConfig {
        let max_key_length = self.max_key_length.unwrap_or(DEFAULT_MAX_KEY_LENGTH);

        let protocol = self.protocol.unwrap_or_default();
        let mut unique_protocol: Vec<Protocol> = Vec::with_capacity(protocol.len());

        for p in protocol {
            if !unique_protocol.contains(&p) {
                unique_protocol.push(p);
            }
        }

        if unique_protocol.is_empty() {
            unique_protocol.push(Protocol::Http);
        }

        let http_enabled = unique_protocol.contains(&Protocol::Http);
        let tcp_enabled = unique_protocol.contains(&Protocol::Tcp);

        let http_host = if http_enabled {
            Some(self.http_host.unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)))
        } else {
            None
        };

        let http_port = if http_enabled {
            Some(self.http_port.unwrap_or(DEFAULT_HTTP_PORT))
        } else {
            None
        };

        let tcp_host = if tcp_enabled {
            Some(self.tcp_host.unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)))
        } else {
            None
        };

        let tcp_port = if tcp_enabled {
            Some(self.tcp_port.unwrap_or(DEFAULT_TCP_PORT))
        } else {
            None
        };

        self.max_key_length = Some(max_key_length);
        self.protocol = Some(unique_protocol);

        self.http_host = http_host;
        self.http_port = http_port;

        self.tcp_host = tcp_host;
        self.tcp_port = tcp_port;

        self
    }
}
impl<'a> From<Cli> for RuntimeConfig {
    fn from(cli: Cli) -> Self {
        Self {
            max_key_length: cli.max_key_length,
            protocol: cli.protocol,
            http_host: cli.http_host,
            http_port: cli.http_port,
            tcp_host: cli.tcp_host,
            tcp_port: cli.tcp_port,
        }
    }
}
