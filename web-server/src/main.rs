use hyperlane::*;

async fn run_server() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0");
    server.port(60000);
    server.log_dir("./logs");
    server.log_size(1_024_000);
    server
        .async_router(
            "/",
            |arc_lock_controller_data: ArcRwLockControllerData| async move {
                let _ = send_response(&arc_lock_controller_data, 200, "hello");
            },
        )
        .await;
    server.listen();
}

#[tokio::main]
async fn main() {
    run_server().await;
}
