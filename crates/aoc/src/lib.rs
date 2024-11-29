use reqwest::{Client, Url};
use std::fs;
use std::sync::Arc;

/// the location where the input files will be locally cached
const DATA_DIR: &str = "Data";
/// the domain for the Advent of Code site
const DOMAIN: &str = "adventofcode.com";

fn get_session_id() -> Result<String, String> {
    println!("{}", std::env::current_dir().unwrap().display());
    fs::read_to_string(".session").map_err(|e| e.to_string())
}

fn build_file_name(year: &str, day: &str) -> String {
    format!("{}.day{}.dat", year, day)
}

/// Retrieves the input from the cache if it exists.
///
/// # Arguments
///
/// * `year` - A string slice that holds the year of the event.
/// * `day` - A string slice that holds the day of the event.
///
/// # Returns
///
/// * `Ok(Some(String))` - If the input file exists and is read successfully.
/// * `Ok(None)` - If the input file does not exist.
/// * `Err(String)` - If there is an error reading the input file or creating the directory.
pub fn get_input_from_cache(year: &str, day: &str) -> Result<Option<String>, String> {
    let input_dir = std::env::current_dir().unwrap().join(DATA_DIR);
    if !input_dir.exists() {
        fs::create_dir(&input_dir).map_err(|e| e.to_string())?;
    }

    let input_file = input_dir.join(build_file_name(year, day));
    return if input_file.exists() {
        println!("loading input from cache");
        let body = fs::read_to_string(input_file).map_err(|e| e.to_string())?;
        Ok(Some(body))
    } else {
        Ok(None)
    };
}
/// Cache the input data for later recall.
///
/// # Arguments
///
/// * `year` - A string slice that holds the year of the event.
/// * `day` - A string slice that holds the day of the event.
/// * `body` - A string slice that holds the input data to be cached.
///
/// # Returns
///
/// * `Ok(())` - If the input is successfully written to the cache.
/// * `Err(String)` - If there is an error writing the input to the cache.
pub fn add_to_cache(year: &str, day: &str, body: &str) -> Result<(), String> {
    let input_dir = std::env::current_dir().unwrap().join(DATA_DIR);
    if !input_dir.exists() {
        fs::create_dir(&input_dir).map_err(|e| e.to_string())?;
    }

    let input_file = input_dir.join(build_file_name(year, day));
    fs::write(input_file, body).map_err(|e| e.to_string())
}

/// Fetches the input from the site for the specified year and day.
///
/// # Arguments
///
/// * `year` - A string slice that holds the year of the event.
/// * `day` - A string slice that holds the day of the event.
/// * `session_id` - A string slice that holds the session ID for authentication.
///
/// # Returns
///
/// * `Ok(String)` - If the input is fetched successfully.
/// * `Err(String)` - If there is an error fetching the input.
pub async fn get_input_from_site(
    year: &str,
    day: &str,
    session_id: &str,
) -> Result<String, String> {
    let path = format!("https://{}/{}/day/{}/input", DOMAIN, year, day);
    println!("Fetching input from {}", path);

    if path.is_empty() {
        return Err("Path is empty".to_string());
    }

    let url = Url::parse("https://adventofcode.com").map_err(|e| e.to_string())?;
    let jar = reqwest::cookie::Jar::default();

    jar.add_cookie_str(
        format!("session={}; Domain={}; Path=/", session_id, DOMAIN).as_str(),
        &url,
    );

    let client = Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client.get(&path).send().await.map_err(|e| e.to_string())?;
    let status = &response.status();
    let body = response.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!(
            "Failed to fetch input from {}.  Response: {}",
            path, body
        ));
    }

    Ok(body)
}
/// Fetches the input for the specified year and day.
///
/// # Arguments
///
/// * `year` - A string slice that holds the year of the event.
/// * `day` - A string slice that holds the day of the event.
///
/// # Returns
///
/// * `Ok(Vec<String>)` - If the input is fetched and parsed successfully.
/// * `Err(String)` - If there is an error fetching or parsing the input.
pub async fn get_input(year: &str, day: &str) -> Result<Vec<String>, String> {
    println!("Fetching input from for AOC {} Day {}", year, day);

    let body = match get_input_from_cache(year, day) {
        Ok(Some(body)) => body,
        Ok(None) => {
            let session_id = get_session_id()?;
            let body = get_input_from_site(year, day, &session_id).await?;
            add_to_cache(year, day, &body)?;
            body
        }
        Err(e) => return Err(e),
    };

    let result: Vec<String> = body.split("\n").map(|s| s.to_string()).collect();

    Ok(result)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = get_input("input.txt", "53616c7465645f5f372e9aeafc01cc4c033858225b31a78b69adeef9c1f4c9e88a31eda8a468b46adb8d8b2e4dadb4e87509e188aae71632f44e7d15783101bd");
//         assert_eq!(result, Ok(Vec::<String>::new()));
//     }
// }
