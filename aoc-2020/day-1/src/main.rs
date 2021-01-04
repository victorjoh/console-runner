use reqwest::blocking;
use reqwest::header;
use std::env;
use std::error::Error;
use std::fs;
use std::io;

const INPUT_CACHE_FILE: &str = "2020-1-input.txt";
const DAY_1_INPUT: &str = "https://adventofcode.com/2020/day/1/input";

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_cached_input().or_else::<Box<dyn Error>, _>(|_| {
        let session = get_session_id(env::args())?;
        let input = download_input(session)?;
        write_cached_input(&input)?;
        Ok(input)
    })?;

    println!("Answer part 1= {}", day1::solve_part_1(&input));
    println!("Answer part 2= {}", day1::solve_part_2(&input));
    Ok(())
}

fn read_cached_input() -> io::Result<String>  {
    let res = fs::read_to_string(INPUT_CACHE_FILE);
    if res.is_ok() {
        println!("successfully read input cache at {}", INPUT_CACHE_FILE);
    } else {
        println!("failed to read input cache at {}", INPUT_CACHE_FILE);
    }
    res
}

fn get_session_id(mut args: env::Args) -> Result<String, &'static str> {
    args.next();
    args.next().ok_or("session is missing from the arguments")
}

fn download_input(aoc_web_session_id: String) -> Result<String, reqwest::Error> {
    println!("downloading input from {}...", DAY_1_INPUT);
    let input = blocking::Client::new()
        .get(DAY_1_INPUT)
        .header(header::COOKIE, format!("session={}", aoc_web_session_id))
        .send()?
        .text()?;
    println!("finished downloading input");
    Ok(input)
}

fn write_cached_input(contents: &str) -> io::Result<()> {
    fs::write(INPUT_CACHE_FILE, contents)?;
    println!("cached input to {}", INPUT_CACHE_FILE);
    Ok(())
}
