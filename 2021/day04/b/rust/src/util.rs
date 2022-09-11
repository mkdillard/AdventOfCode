use hyper::{Client, Request, Method};
use hyper_tls::HttpsConnector;
use std::fs;
//use std::io::Write;

fn load_input_from_cache_if_exists() -> String {
    match fs::read_to_string("input") {
        Ok(s) => {
            println!("Input found in cache.");
            s
        },
        Err(_e) => {
            println!("Input not in cache, attempting to read from adventofcode");
            "".to_string()
        }
    }
}

fn read_session_cookie_from_file() -> String {
    match fs::read_to_string("session") {
        Ok(s) => ( s ),
        Err(_e) => {
            println!("session file not in current, attempting to read from root");
            fs::read_to_string("../../../../session").expect("Unable to recover session")
        }
    }
}

#[tokio::main]
pub async fn fetch_puzzle_input(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let session_cookie = read_session_cookie_from_file();
    let request = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("cookie", &("session=".to_owned() + &session_cookie))
        .body(hyper::Body::empty())
        .unwrap();

    let resp = client.request(request).await?;
    println!("Response Status: {:?}", resp.status());

    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let input = String::from_utf8(body_bytes.to_vec()).expect("Input was not valid utf-b");

    Ok(input)
}

fn cache_input(input: &str) {
    fs::write("input", input).expect("Failed to save input to cache.");
    
}

pub fn get_input(url: &str) -> String{
    let mut input = load_input_from_cache_if_exists();
    if input.chars().count() == 0 {
        input = fetch_puzzle_input(url).expect(&(format!("Unable to retrieve input from {}", url)));
        cache_input(&input);
    }
    input
}

