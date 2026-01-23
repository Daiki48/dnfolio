use std::net::SocketAddr;

use axum::Router;
use axum::routing::get_service;
use socket2::{Domain, Socket, Type};
use tower_http::services::ServeDir;

/// 開始ポート
const BASE_PORT: u16 = 3000;
/// ポート探索の最大試行数
const MAX_PORT_ATTEMPTS: u16 = 10;

pub async fn run() -> anyhow::Result<()> {
    let (socket, addr) = bind_available_port()?;
    socket.listen(128)?;

    let serve_dir = ServeDir::new("dist");
    let app = Router::new().fallback(get_service(serve_dir));

    println!("Listening on http://{addr}");
    println!("Press Ctrl + C for exit");

    let listener: std::net::TcpListener = socket.into();
    listener.set_nonblocking(true)?;
    let tokio_listener = tokio::net::TcpListener::from_std(listener)?;

    axum::serve(tokio_listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

/// 使用可能なポートを探してバインドする
fn bind_available_port() -> anyhow::Result<(Socket, SocketAddr)> {
    for port in BASE_PORT..BASE_PORT + MAX_PORT_ATTEMPTS {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
        socket.set_reuse_address(true)?;
        match socket.bind(&addr.into()) {
            Ok(()) => return Ok((socket, addr)),
            Err(_) => continue,
        }
    }
    anyhow::bail!(
        "ポート {BASE_PORT}~{} が全て使用中です",
        BASE_PORT + MAX_PORT_ATTEMPTS - 1
    )
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Signal received, starting graceful shutdown");
}
