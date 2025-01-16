use hyperlane::*;

async fn run_server() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0");
    server.port(60000);
    server.log_dir("./logs");
    server.log_size(1_024_000);
    server.thread_pool_size(8);
    server
        .async_router("/", |arc_lock_data: ArcRwLockControllerData| async move {
            let mut data: RwLockWriteControllerData = arc_lock_data.write().unwrap();
            let stream: Arc<TcpStream> = data.get_stream().clone().unwrap();
            data.get_mut_response()
                .set_body("hello".into())
                .send(&stream)
                .unwrap();
        })
        .await;
    server.listen();
}

#[tokio::main]
async fn main() {
    run_server().await;
}
