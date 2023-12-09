use clap::Parser;
use color_eyre::{eyre::Context, Result};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

const DAYS: &[fn(String)] = &[
    day01::solution,
    day02::solution,
    day03::solution,
    day04::solution,
    day05::solution,
    day06::solution,
    day07::solution,
    day08::solution,
    day09::solution,
];

#[derive(Debug, clap::Parser)]
struct Args {
    /// The day (1-25) to run
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
    /// The input file to use (or - for stdin), defaults to input/dayXX.txt
    input: Option<String>,
    /// The session cookie to use for downloading input
    #[arg(long, env = "AOC_SESSION_COOKIE")]
    session_cookie: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let input = match args.input.as_deref() {
        Some("-") => std::io::read_to_string(std::io::stdin())?,
        Some(path) => std::fs::read_to_string(path)
            .wrap_err_with(|| format!("Failed to read input from {path}"))?,
        None => {
            let path = format!("input/day{:02}.txt", args.day);
            match std::fs::read_to_string(&path) {
                Ok(input) => input,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    if let Some(session_cookie) = args.session_cookie {
                        download_input(args.day, &session_cookie).wrap_err_with(|| {
                            format!("Failed to download input for day {}", args.day)
                        })?
                    } else {
                        return Err(err)
                            .wrap_err_with(|| format!("Failed to read input from {path} and no session cookie was provided"));
                    }
                }
                Err(err) => {
                    return Err(err).wrap_err_with(|| format!("Failed to read input from {path}"))
                }
            }
        }
    };
    let solution = DAYS
        .get(args.day as usize - 1)
        .unwrap_or_else(|| panic!("Day {} has not been implemented yet", args.day));
    solution(input);
    Ok(())
}

fn download_input(day: u8, session_cookie: &str) -> Result<String> {
    match std::fs::create_dir("input") {
        Err(err) if err.kind() != std::io::ErrorKind::AlreadyExists => {
            return Err(err).wrap_err_with(|| "Failed to create input directory");
        }
        Ok(()) | Err(_) => {}
    }
    let path = format!("input/day{:02}.txt", day);
    match std::fs::read_to_string(&path) {
        Ok(input) => Ok(input),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            let input = reqwest::blocking::Client::new()
                .get(format!("https://adventofcode.com/2023/day/{}/input", day))
                .header("Cookie", format!("session={}", session_cookie))
                .send()?
                .error_for_status()?
                .text()?;
            std::fs::write(&path, &input)
                .wrap_err_with(|| format!("Failed to write downloaded input to {path}"))?;
            Ok(input)
        }
        Err(err) => Err(err).wrap_err_with(|| format!("Failed to read input from {path}")),
    }
}
