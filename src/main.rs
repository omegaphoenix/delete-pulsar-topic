mod auth;
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
    pub oauth: auth::OAuth,
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
                oauth,
            },
    } = config::load().expect("Unable to load config");
    let client = Client::new();

    // Get OAuth token
    let token = auth::get_auth_token(&client, oauth).await?;

    // Create a new header map
    let mut headers = HeaderMap::new();
    headers.insert(
        "Accept",
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    headers.insert("Authorization", format!("Bearer {}", &token).parse()?);

    let hostname = format!("https://{hostname}");
    let path_prefix = format!("/admin/v2/persistent/{tenant}/{namespace}");
    let query_string = "?force=true";

    for topic in topics {
        let topic_uri = format!("{hostname}{path_prefix}/{topic}");
        let delete_partitioned_uri = format!("{topic_uri}/partitions{query_string}");
        let mut status = delete(&client, delete_partitioned_uri, headers.clone()).await?;

        if matches!(status, StatusCode::NOT_FOUND) {
            let delete_non_partitioned_uri = format!("{topic_uri}{query_string}");
            log::warn!(
                "404 when hitting partitioned delete route. Trying non-partitioned route now."
            );
            status = delete(&client, delete_non_partitioned_uri, headers.clone()).await?;
        }

        handle_status(status, &namespace, &topic);
    }

    Ok(())
}

async fn delete(
    client: &Client,
    uri: String,
    headers: HeaderMap,
) -> Result<StatusCode, Box<dyn std::error::Error>> {
    log::info!("{uri}");
    let response = client.delete(uri).headers(headers.clone()).send().await?;
    Ok(response.status())
}

fn handle_status(status: StatusCode, namespace: &str, topic: &str) {
    println!("Status: {status:?} {namespace} {topic}");
    match status {
        StatusCode::NO_CONTENT => println!("Successfully deleted {}/{}", namespace, topic),
        StatusCode::UNAUTHORIZED => {
            println!(
                "Unauthorized
                - Are your OAuth credentials correct?
                - Does your service account have admin permissions?
                - Check your client_id, client_secret, and audience configuration."
            )
        }
        StatusCode::FORBIDDEN => println!("Forbidden: Does your service account have admin permissions?"),
        StatusCode::NOT_FOUND => println!("Not found: Topic was not found."),
        _ => println!("Unexpected status code - please ask for help so we can document this error"),
    }
}
