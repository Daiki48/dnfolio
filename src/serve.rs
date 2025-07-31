use std::net::SocketAddr;

use axum::Router;
use axum::routing::get_service;
use socket2::{Domain, Socket, Type};
use tower_http::services::ServeDir;

pub async fn run() -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    socket.bind(&addr.into())?;
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
