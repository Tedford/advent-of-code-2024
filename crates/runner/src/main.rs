use aoc;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <year> <day> <session_id>", args[0]);
        std::process::exit(1);
    }

    let year = &args[1];
    let day = &args[2];
    let session_id = aoc::get_session_id().unwrap();
    println!("Using session id: {}", session_id);
    let input = aoc::get_input(format!("https://adventofcode.com/{}/day/{}/input",year,day).as_str(),&session_id).await;
    match input {
        Ok(lines) => println!("Success\n{}", lines.join("\n")),
        Err(e) => println!("Error: {}", e),
    }
}

