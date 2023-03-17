use hyper::Uri;
use hyper::{ Body, Client, client::HttpConnector};
use std::time::*;

#[tokio::main]
async fn main() {
    spawn_task(50000).await;
}

async fn spawn_task(times: i32) {
    let mut durations: Vec<Duration> = vec![];
    let mut i = 0;
    let client = Client::new();
    while i < times {
        println!("{}", i);
        let start = Instant::now();
        make_get_request(client.clone()).await;
        let elapsed = start.elapsed();
        durations.push(elapsed);
        i = i + 1;
    }
    println!("{:?}", durations);
    calculate_avg_response_time(durations.clone(), times);
}

fn calculate_avg_response_time(durations: Vec<Duration>, iterations: i32) {
    let mut total: Duration = Duration::new(0, 0);
    for duration in durations {
        total = duration + total;
    }
    let avg_resp_time = total / iterations as u32;
    println!("average response time was: {:?}", avg_resp_time);
    println!("total time taken for all requests to complete was: : {:?}", total);
}

async fn make_get_request(client: Client<HttpConnector, Body>) {
        let handle = tokio::spawn(async move {
        let url: hyper::Uri = Uri::from_static("http://127.0.0.1:8080/person");
        let future = client.get(url).await;
        future.unwrap()
        });
        let out = handle.await;
        println!("GOT {:?}", out);
}