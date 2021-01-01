use reqwest::blocking;
use reqwest::header;
use std::env;
use std::error::Error;
use std::fs;

const INPUT_CACHE_FILE: &str = "2020-1-input.txt";
const DAY_1_INPUT: &str = "https://adventofcode.com/2020/day/1/input";

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let session = args.get(1).expect("session is missing from the arguments");

    println!("looking for cached input at {}", INPUT_CACHE_FILE);
    let input = fs::read_to_string(INPUT_CACHE_FILE).unwrap_or_else(|_| {
        let input = download_input(session).unwrap();
        fs::write(INPUT_CACHE_FILE, &input).unwrap();
        input
    });
    println!("Input = {}", input);
    Ok(())
}

fn download_input(aoc_web_session_id: &str) -> reqwest::Result<String> {
    println!("downloading input from {} ...", DAY_1_INPUT);
    let input = blocking::Client::new()
        .get(DAY_1_INPUT)
        .header(header::COOKIE, format!("session={}", aoc_web_session_id))
        .send()?
        .text();
    println!("finished downloading input");
    input
}
