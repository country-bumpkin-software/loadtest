use std::fs::OpenOptions;
use hyper::{client::HttpConnector, Body, Client, StatusCode};
use std::time::*;
use std::env;
use std::env::args;

type ReqClient = Client<HttpConnector, Body>;
#[derive(Debug)]
struct RequestMetrics {
    task: Option<u64>,
    number_of_requests: u64,
    connection_duration: Duration,
    successful_requests: u64,
    rate_of_requests: Duration,
    url: Option<String>

}

impl RequestMetrics {
    fn new() -> Self {
        Self {
            task: None,
            number_of_requests: 0,
            successful_requests: 0,
            connection_duration: Duration::new(0,0),
            rate_of_requests: Duration::new(0,0),
            url: Some("http://127.0.0.1:8080/person".to_string()),
        }
    }
    fn rate_of_requests_by_connection(&mut self) {
        self.rate_of_requests =  self.connection_duration / self.number_of_requests as u32
    }
}

// #[derive(Debug)]
// struct RequestOptions {
//     url: Option<String>,
//     client: Option<ReqClient>,
//     iterations: Option<u64>,
//     task: Option<u64>,
//     request_info: Option<RequestMetrics>
//
// }
// impl RequestOptions {
//     fn new() -> Self {
//         RequestOptions{
//             url:None,
//             client: None,
//             iterations: None,
//             task: None,
//             request_info: None
//         }
//     }
// }

#[tokio::main]
async fn main() {

    let iterations = 10000;
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

async fn send_request(client: ReqClient, iterations: u64, task: u64) -> RequestMetrics{
    let task_start = Instant::now();
    let mut count = 0;
    let mut request_info = RequestMetrics::new();

    while count < iterations {
        let url = hyper::Uri::from_static("http://localhost:8088/dinosaurs/era/cretaceous");
        // http://localhost:8088/dinosaurs/era/cretaceous
        let resp = client.get(url).await;
        // we can verify that there are concurrent requests going out by merely logging the
        // task, along side what iteration of request we are sending. This way we can see the
        // requests are interleaved and not being sent sequentially one task followed by another.
        // println!(
        //     "task: {} Iteration {} received status code: {:?}",
        //     task,
        //     count,
        //     resp.unwrap().status()
        // );
        count = count + 1;
        if resp.unwrap().status() == StatusCode::OK {
            request_info.successful_requests = request_info.successful_requests + 1;
        }
        request_info.task = Some(task);
        request_info.number_of_requests = count;


    }
    let task_finish = task_start.elapsed();
    request_info.connection_duration = task_finish;
    request_info.rate_of_requests_by_connection();
    println!("{:?}", request_info);
    request_info
}
// wrc appears to be more than ten times faster, presumably because it is using multiple threads?
// In production it the results from the current solution may not be realistic simulation of
// real load in a high traffic production environment.  Also this is testing the localhost which isnt realistic
// of a real production environment.  The current solution doesnt support https also.
fn calculate_avg_response_time(durations: Vec<Duration>, iterations: u64) {
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