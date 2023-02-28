use reqwest::Client;
use std::env;

async fn run(endpoint: &str) {
    let client = Client::builder().build().unwrap();

    let res = client.get(endpoint).send().await;

    if res.is_err() {
        panic!("Can't reach route {}", endpoint);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Too many arguments!")
    }

    let endpoint = args.last().unwrap();

    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap()
        .block_on(run(endpoint))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_reach_google() {
        run("http://google.com").await
    }

    #[tokio::test]
    #[should_panic]
    async fn cant_reach_nonsense() {
        run("http://asdqeqweqweqweqwe.local/qweqweqweqwewqe").await
    }
}
