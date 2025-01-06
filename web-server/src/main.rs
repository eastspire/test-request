use hyperlane::*;

fn run_server() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0");
    server.port(8080);
    server.thread_pool_size(100);
    server.log_dir("./logs");
    server.log_size(1_024_000);
    server.router("/", |data| {
        let stream = data.get_stream().clone().unwrap();
        let _ = data
            .get_mut_response()
            .set_body("hello".into())
            .set_header("server", "test")
            .send(&stream);
    });
    server.listen();
}

fn main() {
    run_server();
}
