use hyper::{Body, Request, Response, Server, Client, Method};
use hyper::service::{make_service_fn, service_fn};
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use hyper_tls::HttpsConnector;
use std::fs;

#[derive(Deserialize, Debug)]
struct Config {
    endpoint: String,
    webhook_url: String,
    headers_to_body: Vec<String>,
}

async fn proxy_handler(
    req: Request<Body>,
    config: Arc<Config>,
) -> Result<Response<Body>, Infallible> {
    if req.uri().path() == config.endpoint && req.method() == Method::POST {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        println!("Listening on {}", &config.webhook_url);


        // Create a new request to the webhook URL
        let mut webhook_request = Request::builder()
            .method(Method::POST)
            .uri(&config.webhook_url);

        // Extract headers to include in the body
        let mut body_content = String::new();
        for header_name in &config.headers_to_body {
            if let Some(value) = req.headers().get(header_name) {
                body_content.push_str(&format!(
                    "{}: {}\n",
                    header_name,
                    value.to_str().unwrap_or("")
                ));
            }
        }

        // Make the webhook request with the extracted headers in the body
        let webhook_request = webhook_request
            .body(Body::from(body_content))
            .expect("Failed to create request");

        match client.request(webhook_request).await {
            Ok(_response) => Ok(Response::new(Body::from("Webhook call succeeded!"))),
            Err(_e) => {
                println!("error {}", _e);
                return Ok(Response::new(Body::from("Failed to call webhook!")))
            },
        }
    } else {
        Ok(Response::new(Body::from("Invalid endpoint or method")))
    }
}

#[tokio::main]
async fn main() {
    let config_path = "config.json";

    let config_data = fs::read_to_string(config_path)
        .expect("Unable to read config file");

    // Parse the config at runtime
    let config: Config = serde_json::from_str(&config_data)
        .expect("Invalid config format");
    let config = Arc::new(config);

    // Define the address and port
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(move |_| {
        let config = Arc::clone(&config);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                proxy_handler(req, Arc::clone(&config))
            }))
        }
    });

    // Start the server
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    // Run the server
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

