mod config;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
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

    let hostname = format!("https://{}", hostname);
    let path_prefix = format!("/admin/v2/persistent/{}/{}", tenant, namespace);

    for topic in topics {
        let uri = format!("{}{}/{}?force=true", hostname, path_prefix, topic);
        let response = client.delete(uri).headers(headers.clone()).send().await?;
        println!("Status: {:?} {} {}", response.status(), namespace, topic);
    }

    Ok(())
}
