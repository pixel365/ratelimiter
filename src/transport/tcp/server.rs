use crate::app::App;
use crate::config::cli::Protocol;
use crate::config::helpers::invalid_cfg;
use crate::core::{
    constants::{DEFAULT_KEY_LENGTH, PONG},
    limiter::Limiter,
};
use crate::transport::tcp::protocol::{parse_command, Command};
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
};
use tokio_util::sync::CancellationToken;

pub async fn run(app: App, stop: CancellationToken) -> std::io::Result<()> {
    let Some(protocols) = app.cfg.protocol.as_ref() else {
        return Err(invalid_cfg("protocol is not finalized"));
    };

    if !protocols.contains(&Protocol::Tcp) {
        tracing::info!("TCP protocol is disabled");
        return Ok(());
    }

    let Some(host) = app.cfg.tcp_host else {
        return Err(invalid_cfg("tcp_host is not finalized"));
    };

    let Some(port) = app.cfg.tcp_port else {
        return Err(invalid_cfg("tcp_port is not finalized"));
    };

    let addr = SocketAddr::new(host, port);
    let listener = TcpListener::bind(addr).await?;

    loop {
        tokio::select! {
            _ = stop.cancelled() => {
                tracing::info!("TCP server stopped");
                return Ok(());
            }
            res = listener.accept() => {
                let (stream, addr) = res?;
                let app = app.clone();
                let stop = stop.clone();

                tokio::spawn(async move {
                    tracing::debug!("tcp connection from: {:?}", addr);
                    handle_conn(stream, app, stop).await;
                    tracing::debug!("tcp connection closed: {:?}", addr);
                });
            }
        }
    }
}

async fn handle_conn(stream: tokio::net::TcpStream, app: App, stop: CancellationToken) {
    let (read_half, mut write_half) = stream.into_split();
    let reader = BufReader::new(read_half);
    let mut lines = reader.lines();

    loop {
        tokio::select! {
            _ = stop.cancelled() => {
                break;
            }
            line = lines.next_line() => {
                let Ok(opt) = line else {break;};
                let Some(line) = opt else {break;};

                let response = match parse_command(&line) {
                    Ok(Command::Pong) => format!("{PONG}\n"),
                    Ok(Command::Check(input)) => {
                        let max_key_length = app.cfg.max_key_length.unwrap_or(DEFAULT_KEY_LENGTH);

                        match input.validate(max_key_length) {
                            Err(e) => format!("ERR {e:?}\n"),
                            Ok(()) => {
                                let res = app.limiter.check(input);
                                format!("OK {} {} {}\n", res.allowed, res.remaining, res.reset_ms)
                            }
                        }
                    }
                    Err(e) => format!("ERR {e}\n"),
                };

                if let Err(_) = write_half.write_all(response.as_bytes()).await {
                    break;
                }
            }
        }
    }

    let _ = write_half.shutdown().await;
}
