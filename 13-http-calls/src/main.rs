//! Blocking HTTP is useful for simple programs that do not need an async runtime.

use reqwest::blocking::{Client, Response};
use reqwest::header::{ACCEPT, HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::time::Duration;

type DemoResult = Result<(), Box<dyn Error>>;

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn run(number: u8, title: &str, example: fn() -> DemoResult) {
    heading(number, title);
    if let Err(error) = example() {
        // A missing network should not prevent the learner from seeing later examples.
        println!("example could not complete: {error}");
    }
}

fn client_with_timeout(timeout: Duration) -> Result<Client, reqwest::Error> {
    Client::builder()
        .timeout(timeout)
        .user_agent("rust-learning-workspace/0.1")
        .build()
}

fn checked(response: Response) -> Result<Response, reqwest::Error> {
    response.error_for_status()
}

fn main() {
    run(1, "basic GET", example_01_basic_get);
    run(2, "query parameters", example_02_query_parameters);
    run(3, "request headers", example_03_request_headers);
    run(4, "HTTP status handling", example_04_status_handling);
    run(5, "dynamic JSON", example_05_dynamic_json);
    run(6, "typed JSON response", example_06_typed_json);
    run(7, "POST JSON (simulated)", example_07_post_json);
    run(
        8,
        "reusable client and timeout",
        example_08_reusable_client_and_timeout,
    );
    run(9, "connection-error handling", example_09_connection_error);
}

fn example_01_basic_get() -> DemoResult {
    let client = client_with_timeout(Duration::from_secs(5))?;
    let response = checked(client.get("https://httpbin.org/get").send()?)?;
    println!(
        "status={}; content-type={:?}",
        response.status(),
        response.headers().get("content-type")
    );
    Ok(())
}

fn example_02_query_parameters() -> DemoResult {
    let client = client_with_timeout(Duration::from_secs(5))?;
    let response: Value = checked(
        client
            .get("https://httpbin.org/get")
            .query(&[("language", "rust"), ("level", "beginner")])
            .send()?,
    )?
    .json()?;
    println!("server received args={}", response["args"]);
    Ok(())
}

fn example_03_request_headers() -> DemoResult {
    let client = client_with_timeout(Duration::from_secs(5))?;
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("rust-header-example/1.0"),
    );
    let response: Value = checked(
        client
            .get("https://httpbin.org/headers")
            .headers(headers)
            .send()?,
    )?
    .json()?;
    println!("echoed headers={}", response["headers"]);
    Ok(())
}

fn example_04_status_handling() -> DemoResult {
    let client = client_with_timeout(Duration::from_secs(5))?;
    let response = client.get("https://httpbin.org/status/404").send()?;
    println!("raw status={}", response.status());
    match response.error_for_status() {
        Ok(_) => println!("unexpected success"),
        Err(error) if error.is_status() => println!("expected status error: {error}"),
        Err(error) => return Err(error.into()),
    }
    Ok(())
}

fn example_05_dynamic_json() -> DemoResult {
    let client = client_with_timeout(Duration::from_secs(5))?;
    let post: Value = checked(
        client
            .get("https://jsonplaceholder.typicode.com/posts/1")
            .send()?,
    )?
    .json()?;
    println!("post id={}; title={:?}", post["id"], post["title"].as_str());
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

fn example_06_typed_json() -> DemoResult {
    let client = client_with_timeout(Duration::from_secs(5))?;
    let post: Post = checked(
        client
            .get("https://jsonplaceholder.typicode.com/posts/2")
            .send()?,
    )?
    .json()?;
    println!(
        "typed post {} by user {}: {:?} (body {} bytes)",
        post.id,
        post.user_id,
        post.title,
        post.body.len()
    );
    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NewPost<'a> {
    user_id: u32,
    title: &'a str,
    body: &'a str,
}

fn example_07_post_json() -> DemoResult {
    let client = client_with_timeout(Duration::from_secs(5))?;
    let request = NewPost {
        user_id: 1,
        title: "Learning HTTP",
        body: "JSONPlaceholder simulates this write.",
    };
    let created: Value = checked(
        client
            .post("https://jsonplaceholder.typicode.com/posts")
            .json(&request)
            .send()?,
    )?
    .json()?;
    println!("simulated server response={created}");
    Ok(())
}

fn example_08_reusable_client_and_timeout() -> DemoResult {
    let client = client_with_timeout(Duration::from_millis(300))?;
    let result = client.get("https://httpbin.org/delay/2").send();
    match result {
        Ok(response) => println!("server replied before timeout: {}", response.status()),
        Err(error) if error.is_timeout() => println!("request timed out as configured"),
        Err(error) => return Err(error.into()),
    }
    Ok(())
}

fn example_09_connection_error() -> DemoResult {
    let client = client_with_timeout(Duration::from_millis(500))?;
    match client.get("http://127.0.0.1:9/unavailable").send() {
        Ok(response) => println!("unexpected response: {}", response.status()),
        Err(error) if error.is_connect() || error.is_timeout() => {
            println!("connection failure handled: {error}")
        }
        Err(error) => return Err(error.into()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn client_can_be_built_without_network_access() {
        assert!(client_with_timeout(Duration::from_secs(1)).is_ok());
    }

    #[test]
    fn post_json_has_expected_field_names() {
        let post = NewPost {
            user_id: 7,
            title: "title",
            body: "body",
        };
        let value = serde_json::to_value(post).unwrap();
        assert_eq!(value["userId"], 7);
    }

    /// Starts a tiny, one-request HTTP fixture on a random local port.
    ///
    /// It exists only during a test, needs no external service, and lets the
    /// request/error handling code be exercised deterministically.
    fn serve_once(status: &str, body: &str, delay: Duration) -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let response = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let worker = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            // Consume the complete request headers before writing the response.
            let reader_stream = stream.try_clone().unwrap();
            for line in BufReader::new(reader_stream).lines() {
                if line.unwrap().is_empty() {
                    break;
                }
            }
            thread::sleep(delay);
            // A timeout test may close the socket before this write, which is expected.
            let _ = stream.write_all(response.as_bytes());
        });
        (format!("http://{address}"), worker)
    }

    #[test]
    fn successful_response_decodes_typed_json() {
        let body = r#"{"userId":1,"id":2,"title":"test","body":"offline"}"#;
        let (url, fixture) = serve_once("200 OK", body, Duration::ZERO);
        let post: Post = checked(
            client_with_timeout(Duration::from_secs(1))
                .unwrap()
                .get(url)
                .send()
                .unwrap(),
        )
        .unwrap()
        .json()
        .unwrap();
        fixture.join().unwrap();
        assert_eq!(post.id, 2);
    }

    #[test]
    fn non_success_status_becomes_an_error() {
        let (url, fixture) = serve_once("404 Not Found", "{}", Duration::ZERO);
        let response = client_with_timeout(Duration::from_secs(1))
            .unwrap()
            .get(url)
            .send()
            .unwrap();
        let error = checked(response).unwrap_err();
        fixture.join().unwrap();
        assert!(error.is_status());
    }

    #[test]
    fn malformed_json_becomes_a_decode_error() {
        let (url, fixture) = serve_once("200 OK", "{not-json", Duration::ZERO);
        let error = checked(
            client_with_timeout(Duration::from_secs(1))
                .unwrap()
                .get(url)
                .send()
                .unwrap(),
        )
        .unwrap()
        .json::<Value>()
        .unwrap_err();
        fixture.join().unwrap();
        assert!(error.is_decode());
    }

    #[test]
    fn slow_response_respects_the_timeout() {
        let (url, fixture) = serve_once("200 OK", "{}", Duration::from_millis(100));
        let error = client_with_timeout(Duration::from_millis(10))
            .unwrap()
            .get(url)
            .send()
            .unwrap_err();
        fixture.join().unwrap();
        assert!(error.is_timeout());
    }
}
