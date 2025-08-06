# pulsar-delete-topic
Delete pulsar topics

## Setup

### Prerequisite
- [Install Rust](https://www.rust-lang.org/tools/install) `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- Fetch an admin oauth credentials from the cloud UI:
    - Sign in to [Streamnative Cloud](https://auth.streamnative.cloud/u/login/identifier) using your Google account
    - Select the organization
    - Go to [Accounts and Accesses](https://console.streamnative.cloud/service-accounts) under your profile in the upper right hand corner
    - Click on Service Accounts:
        - Select any admin service account and click on `Download OAuth2 Key`
        - Copy the client_id, client_secret, client_email, and issuer_url and paste it in your local `config.toml`


### To Run
- `cp config.sample.toml config.toml`
- Fill in hostname, tenant, namespace, topics, and audience in the `config.toml`
- `RUST_LOG=info cargo run`

## Success
- `Status: 204 [namespace] [topic]`
- No Content means your request was successful
- Use a [Pulsar reader](https://github.com/omegaphoenix/pulsar-rs-reader) to confirm that there is no data on your topic


## Error(s)
- `Status: 401 [namespace] [topic]`
- Unauthorized
  - Are you oauth credentials for an admin account?
  - Did you update your audience to match the host?

- `Status: 403 [namespace] [topic]`
- Forbidden - Your credentials are valid but are not for an admin account

- `Status: 404 [namespace] [topic]`
- Past error - Topic may be partitioned and we are hitting the non-partitioned route or vice versa. This should be fixed now.
- Not Found - Topic was not found.
