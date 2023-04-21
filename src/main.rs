use hyper::{client::HttpConnector, Body, Client, StatusCode};
use std::time::*;
// use std::env::args;

type ReqClient = Client<HttpConnector, Body>;
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct RequestOptions {
    url: String,
    iterations: u64,
    task: u64,
    request_info: RequestMetrics

}
impl RequestOptions {
    fn new() -> Self {
        RequestOptions{
            url:"".to_string(),
            iterations: 0,
            task: 0,
            request_info: RequestMetrics::new()
        }
    }
}

#[tokio::main]
async fn main() {

    let iterations = 1000;
    let tasks = 12;
    let mut count = 0;
    let mut options = RequestOptions::new();

    let mut connections: Vec<ReqClient> = Vec::new();
    let mut handles = Vec::new();
    let mut durations: Vec<Duration> = vec![];
    let start = Instant::now();
    let mut metrics = RequestMetrics::new();
    options.iterations = 1000;
    options.task = tasks;

    for _ in 0..tasks {
        connections.push(ReqClient::new());
    }
    for client in connections {
        options.task = count;
        let mut options_clone  = options.clone();
        let handle = tokio::spawn(async move {
            send_request(client, &mut options_clone).await;
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

async fn send_request(client: ReqClient, options: &mut RequestOptions) -> Vec<RequestMetrics>{
    let mut vec_of_metrics = Vec::new();
    let mut metrics = RequestMetrics::new();
    let task_start = Instant::now();
    let mut count = 0;
    let request_info = RequestMetrics::new();
    let url = hyper::Uri::from_static("http://localhost:8088/dinosaurs/era/cretaceous");
    while count < options.iterations {

        // http://localhost:8088/dinosaurs/era/cretaceous
        let resp = client.get(url.clone()).await;
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
            metrics.successful_requests = metrics.successful_requests + 1;
        }
        metrics.task = Some(options.task);
        metrics.number_of_requests = count;
    }
    let task_finish = task_start.elapsed();
    metrics.connection_duration = task_finish;
    metrics.rate_of_requests_by_connection();

    vec_of_metrics.push(metrics);
    println!("{:?}", vec_of_metrics);
    vec_of_metrics
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