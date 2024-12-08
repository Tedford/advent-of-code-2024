use aoc;
use std::process;
use url::Url;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    if args.len() < 3 {
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
            let start_time = std::time::Instant::now();
            let (part1, part2) = match day.as_str() {
                "1" => (day1::part1(&lines) as i64, day1::part2(&lines) as i64),
                "2" => (day2::part1(&lines) as i64, day2::part2(&lines) as i64),
                "3" => (day3::part1(&lines) as i64, day3::part2(&lines) as i64),
                "4" => (day4::part1(&lines) as i64, day4::part2(&lines) as i64),
                "5" => (day5::part1(&lines) as i64, day5::part2(&lines) as i64),
                "6" => (day6::part1(&lines) as i64, day6::part2(&lines) as i64),
                "7" => (day7::part1(&lines), day7::part2(&lines)),
                "9" => (day9::part1(&lines), day9::part2(&lines)),
                _ => {
                    println!("Day {} not implemented", day);
                    process::exit(3);
                }
            };
            let duration = start_time.elapsed();
            println!("Elapsed: {:?}", duration);
            println!("Part 1: {}", part1);
            println!("Part 2: {}", part2);
        }
        Err(e) => println!("Error: {}", e),
    }
}
