mod config;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, StatusCode};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub pulsar: PulsarConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PulsarConfig {
    pub hostname: String,
    pub tenant: String,
    pub namespace: String,
    pub topics: Vec<String>,
    pub token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let Config {
        pulsar:
            PulsarConfig {
                hostname,
                tenant,
                namespace,
                topics,
                token,
            },
    } = config::load().expect("Unable to load config");
    let client = Client::new();

    // Create a new header map
    let mut headers = HeaderMap::new();
    headers.insert(
        "Accept",
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    headers.insert("Authorization", format!("Bearer {}", &token).parse()?);

    let hostname = format!("https://{hostname}");
    let path_prefix = format!("/admin/v2/persistent/{tenant}/{namespace}");

    for topic in topics {
        let topic_uri = format!("{hostname}{path_prefix}/{topic}");
        let query_string = "?force=true";
        let uri = format!("{topic_uri}{query_string}");
        println!("{uri}");
        let response = client.delete(uri).headers(headers.clone()).send().await?;
        let mut status = response.status();

        if matches!(status, StatusCode::NOT_FOUND) {
            let partitioned_uri = format!("{topic_uri}/partitions{query_string}");
            println!("404 when hitting non-partitioned delete route. Trying partitioned route now: {partitioned_uri}");
            let response = client
                .delete(partitioned_uri)
                .headers(headers.clone())
                .send()
                .await?;
            status = response.status();
        }
        handle_status(status, &namespace, &topic);
    }

    Ok(())
}

fn handle_status(status: StatusCode, namespace: &str, topic: &str) {
    println!("Status: {status:?} {namespace} {topic}");
    match status {
        StatusCode::NO_CONTENT => println!("Successfully deleted {}/{}", namespace, topic),
        StatusCode::UNAUTHORIZED => {
            println!(
                "Unauthorized
                - Is your token an admin token?
                - Is it a new token? Streamnative Cloud tokens expire after 7 days.
                - Did you select the correct Pulsar cluster before generating your token?"
            )
        }
        StatusCode::FORBIDDEN => println!("Forbidden: Is your token an admin token?"),
        StatusCode::NOT_FOUND => println!("Not found: Topic was not found."),
        _ => println!("Unexpected status code - please ask for help so we can document this error"),
    }
}
