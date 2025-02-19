use hyperlane::*;

async fn middleware(arc_lock_controller_data: ArcRwLockControllerData) {
    let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data).await;
    let mut response: Response = controller_data.get_response().clone();
    let body: &str = "hello";
    let stream: ArcRwLockStream = controller_data.get_stream().clone().unwrap();
    let _: ResponseResult = response
        .set_body(body)
        .set_status_code(200)
        .set_header(CONNECTION, CONNECTION_KEEP_ALIVE)
        .send(&stream)
        .await;
}

async fn run_server() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0").await;
    server.port(60000).await;
    server.log_dir("./logs").await;
    server.log_interval_millis(1_000_000_000).await;
    server.middleware(middleware).await;
    server.listen().await;
}

#[tokio::main]
async fn main() {
    run_server().await;
}
