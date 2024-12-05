use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use std::error::Error;

fn fetch_data(url: &str, cookie: &str) -> Result<String, Box<dyn Error>> {
    // Create a reqwest client
    let client = Client::new();

    // Create headers
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(cookie)?);

    // Send the request
    let response = client
        .get(url)
        .headers(headers)
        .send()?
        .text()?; // Fetch the response text

    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/3/input";
    let cookie = "session=53616c7465645f5f147d26c9d5e6e83542b670ed4ccb7a466371b2d9eabc80b28162013dd94a67a431b557a9e021e0be5be55300572e4f430276bf3d4ac88c13";

    let data = match fetch_data(url, cookie) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error fetching data: {}", err); 
            return Err(err);
        },
    };

    println!("{}", data);

    Ok(())
}
