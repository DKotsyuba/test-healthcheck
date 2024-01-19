use std::env;
use std::time::Duration;
use std::thread;
use reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <interval_in_seconds> <url>", args[0]);
        std::process::exit(1);
    }

    let interval = args[1].parse::<u64>().unwrap_or_else(|_| {
        eprintln!("Error: Interval must be an integer");
        std::process::exit(1);
    });

    let url = &args[2];
    match reqwest::Url::parse(url) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("URL parsing error");
            std::process::exit(1);
        }
    }

    println!("Started health checks for {} every {} second'", url, interval);

    loop {
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Checking '{}'. Result: OK(200)", url);
                } else {
                    println!("Checking '{}'. Result: ERR({})", url, response.status().as_u16());
                }
            }
            Err(error) => {
                eprintln!("Error making request: {}", error);
                break;
            }
        }
        thread::sleep(Duration::from_secs(interval))
    }

}