use aoc;
use std::process;
use url::Url;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <year> <day>", args[0]);
        process::exit(1);
    }

    let session_id = match aoc::session::get_session_id(&std::env::current_dir().unwrap()) {
        Some(id) => id,
        None => {
            println!("Session ID not found. Please create a .session file in the current directory with your session ID.");
            process::exit(2);
        }
    };

    let year = &args[1];
    let day = &args[2];
    let context = aoc::Context {
        url: Url::parse("https://adventofcode.com").unwrap(),
        data_dir: std::env::current_dir().unwrap().join("Data"),
        session_id,
    };
    let input = aoc::get_input(year, day, &context).await;
    match input {
        Ok(lines) => {

            let part1 = day1::part1(&lines);
            println!("Part 1: {}", part1);

            let part2 = day1::part2(&lines);
            println!("Part 2: {}", part2);

    },
        Err(e) => println!("Error: {}", e),
    }
}
