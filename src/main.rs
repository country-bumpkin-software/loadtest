use hyper::Uri;
use hyper::{body, Body, Client, Method, Request, Response, client::HttpConnector};
use std::time::*;
use std::{thread, time};
use std::thread::sleep;

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
        let url: hyper::Uri = Uri::from_static("http://localhost:8088/home");
        let future = client.get(url).await;
        future.unwrap()
        });
        let out = handle.await;
        println!("GOT {:?}", out);
}