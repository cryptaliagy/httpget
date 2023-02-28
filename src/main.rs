use reqwest::Client;
use std::env;

async fn run() {
    let args = env::args();

    if args.len() > 2 {
        panic!("Too many arguments!")
    }

    let endpoint = args.last().unwrap();

    let client = Client::builder().build().unwrap();

    let res = client.get(&endpoint).send().await;

    if res.is_err() {
        panic!("Can't reach route {}", endpoint);
    }
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap()
        .block_on(run())
}
