use std::fs;
use reqwest::{Client, Url};
use std::sync::Arc;

pub fn get_session_id () -> Result<String,String> {
    fs::read_to_string(".session").map_err(|e| e.to_string())
}

pub async fn get_input(path: &str, session_id: &str) -> Result<Vec<String>, String> {
    println!("Fetching input from {}", path);

    if path.is_empty() {
        return Err("Path is empty".to_string());
    }

    let url = Url::parse("https://adventofcode.com").map_err(|e| e.to_string())?;
    let jar = reqwest::cookie::Jar::default();

    jar.add_cookie_str(
        format!("session={}; Domain=adventofcode.com; Path=/", session_id).as_str(),
        &url,
    );

    let client = Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client.get(path).send().await.map_err(|e| e.to_string())?;
    let status = &response.status();
    let body = response.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!(
            "Failed to fetch input from {}.  Response: {}",
            path, body
        ));
    }

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
