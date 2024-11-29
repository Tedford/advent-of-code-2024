use aoc;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <year> <day>", args[0]);
        std::process::exit(1);
    }

    let year = &args[1];
    let day = &args[2];
    let input = aoc::get_input(year, day).await;
    match input {
        Ok(lines) => println!("Success\n{}", lines.join("\n")),
        Err(e) => println!("Error: {}", e),
    }
}
