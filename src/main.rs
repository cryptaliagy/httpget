use reqwest::Client;
use std::{env, process::ExitCode};

async fn run(endpoint: &str) -> Result<(), reqwest::Error> {
    let client = Client::builder().build().unwrap();

    client.get(endpoint).send().await.map(|_| ())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Too many arguments!")
    }

    let endpoint = args.last().unwrap();

    let res = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .expect("Could not build the runtime")
        .block_on(run(endpoint));

    if res.is_ok() {
        ExitCode::from(0)
    } else {
        println!("{}", res.unwrap_err());
        ExitCode::from(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_reach_google() {
        let res = run("http://google.com").await;

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn cant_reach_nonsense() {
        let res = run("http://asdqeqweqweqweqwe.local/qweqweqweqwewqe").await;

        assert!(res.is_err())
    }
}
