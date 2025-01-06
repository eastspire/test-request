use http_request::*;
use hyper::{Client, Uri};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use tokio;

static TIMES: u128 = 10000;
static URL: &str = "http://127.0.0.1:8080/";

#[tokio::main]
async fn hyper() {
    use tokio::spawn;
    let _ = spawn(async {
        let mut total_time: u128 = 0;
        for _i in 0..TIMES {
            let uri: Uri = URL.parse().expect("");
            let client = Client::new();
            let start = Instant::now();
            let _response: Result<hyper::Response<hyper::Body>, hyper::Error> =
                client.get(uri).await;
            let duration: Duration = start.elapsed();
            total_time += duration.as_micros();
        }
        println!("hyper agv time: {} us", total_time / TIMES);
    })
    .await;
}

fn http_request() {
    use std::thread::spawn;
    spawn(|| {
        let mut total_time: u128 = 0;
        for _i in 0..TIMES {
            let start: Instant = Instant::now();
            let mut _request_builder = RequestBuilder::new()
                .post(URL)
                .unredirect()
                .buffer(100)
                .http1_1_only()
                .build();
            let _res = _request_builder.send();
            let duration: Duration = start.elapsed();
            total_time += duration.as_micros();
        }
        println!("http-request agv time: {} us", total_time / TIMES);
    });
}

/// 阿帕奇
/// http-request agv time: 33 us
/// hyper agv time: 557 us
///
/// hyperlane
/// http-request agv time: 121 us
/// hyper agv time: 215 us
fn main() {
    http_request();
    hyper();
    sleep(Duration::from_secs(1000));
}
