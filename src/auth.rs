use reqwest::Client;
use serde::{Deserialize, Serialize};

const OAUTH_URL: &str = "https://auth.streamnative.cloud/oauth/token";
const GRANT_TYPE: &str = "client_credentials";

#[derive(Clone, Debug, Deserialize)]
pub struct OAuth {
    pub client_id: String,
    pub client_secret: String,
    pub client_email: String,
    pub issuer_url: String,
    pub audience: String,
}

pub async fn get_auth_token(client: &Client, oauth: OAuth) -> Result<String, Box<dyn std::error::Error>> {
    let payload: AuthRequestPayload = oauth.into();
    let payload = serde_json::to_vec(&payload)?;

    let response = client
        .post(OAUTH_URL)
        .header("content-type", "application/json")
        .body(payload)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("OAuth request failed with status: {}", response.status()).into());
    }

    let auth_response: AuthResponse = response.json().await?;
    Ok(auth_response.access_token)
}

#[derive(Clone, Serialize)]
struct AuthRequestPayload {
    r#type: String,
    client_id: String,
    client_secret: String,
    client_email: String,
    issuer_url: String,
    grant_type: String,
    audience: String,
}

impl From<OAuth> for AuthRequestPayload {
    fn from(oauth: OAuth) -> Self {
        let OAuth {
            client_id,
            client_secret,
            client_email,
            issuer_url,
            audience,
        } = oauth;
        Self {
            r#type: "sn_service_account".into(),
            client_id,
            client_secret,
            client_email,
            issuer_url,
            grant_type: GRANT_TYPE.into(),
            audience,
        }
    }
}

#[derive(Clone, Deserialize)]
struct AuthResponse {
    access_token: String,
}
