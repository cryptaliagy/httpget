use std::{env, process::ExitCode};

#[inline]
fn run(endpoint: &str) -> Result<minreq::Response, minreq::Error> {
    minreq::get(endpoint).send()
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("Too many arguments!")
    }

    let endpoint = if args.len() == 2 {
        args.last().unwrap().to_owned()
    } else {
        env::var("HTTPGET_ENDPOINT").expect("Environment variable HTTPGET_ENDPOINT not set")
    };

    let res = run(&endpoint);

    if res.is_err() {
        println!("{}", res.unwrap_err());
        return ExitCode::from(1);
    }

    let code = res.unwrap().status_code;

    if code > 299 {
        println!("Received status code {}", code);
        return ExitCode::from(1);
    }

    ExitCode::from(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_reach_google() {
        let res = run("http://google.com");

        assert!(res.is_ok())
    }

    #[test]
    fn cant_reach_nonsense() {
        let res = run("http://asdqeqweqweqweqwe.local/qweqweqweqwewqe");

        assert!(res.is_err())
    }
}
