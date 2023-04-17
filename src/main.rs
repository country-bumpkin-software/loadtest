use hyper::{client::HttpConnector, Body, Client};
use std::time::*;

type ReqClient = Client<HttpConnector, Body>;

#[tokio::main]
async fn main() {
    let iterations = 51200;
    let tasks = 12;
    let mut count = 0;
    let mut connections: Vec<ReqClient> = Vec::new();
    let mut handles = Vec::new();

    let mut durations: Vec<Duration> = vec![];
    let start = Instant::now();
    for _ in 0..tasks {
        connections.push(ReqClient::new());
    }
    for client in connections {
        let handle = tokio::spawn(async move {
            send_request(client, iterations, count).await;
        });
        handles.push(handle);
        count = count + 1;
    }
    for handle in handles {
        let out = handle.await.unwrap();
        println!("GOT {:?}", out);
    }
    let elapsed = start.elapsed();
    durations.push(elapsed);
    calculate_avg_response_time(durations.clone(), iterations);
}

async fn send_request(client: ReqClient, iterations: i32, task: i32) {
    let mut count = 0;

    while count < iterations {
        let url = hyper::Uri::from_static("http://127.0.0.1:8080/person");

        let resp = client.get(url).await;
        // we can verify that there are concurrent requests going out by merely logging the
        // task, along side what iteration of request we are sending. This way we can see the
        // requests are interleaved and not being sent sequentially one task followed by another.
        println!(
            "task: {} Iteration {} received status code: {:?}",
            task,
            count,
            resp.unwrap().status()
        );
        count = count + 1;
    }
}
// wrc appears to be more than ten times faster, presumably because it is using multiple threads?
// In production it the results from the current solution may not be realistic simulation of
// real load in a high traffic production environment.  Also this is testing the localhost which isnt realistic
// of a real production environment.  The current solution doesnt support https also.
fn calculate_avg_response_time(durations: Vec<Duration>, iterations: i32) {
    let mut total: Duration = Duration::new(0, 0);
    for duration in durations {
        total = duration + total;
    }
    let avg_resp_time = total / iterations as u32;
    println!("average response time was: {:?}", avg_resp_time);
    println!(
        "total time taken for all requests to complete was: : {:?}",
        total
    );
}