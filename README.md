# pulsar-delete-topic
Delete pulsar topics

## Setup

### Prerequisite
- [Install Rust](https://www.rust-lang.org/tools/install) `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- Fetch an admin token from the cloud UI:
    - Sign in to [Streamnative Cloud](https://auth.streamnative.cloud/u/login/identifier) using your Google account
    - Select the organization
    - Change your host/cluster to the one that you want to delete from.
        - The tenant/namespace does not matter since tokens are generated per service account and cluster/host.
    - Click on Service Accounts:
        - Select any admin service account and click on `Generate new token`
        - Copy the token and paste it in your local `config.toml`
        - Note that these tokens expire after 7 days


### To Run
- `cp config.sample.toml config.toml`
- Fill in hostname, tenant, namespace, token, and topics in the `config.toml`
- `RUST_LOG=info cargo run`

## Success
- `Status: 204 [namespace] [topic]`
- No Content means your request was successful
- Use a [Pulsar reader](https://github.com/omegaphoenix/pulsar-rs-reader) to confirm that there is no data on your topic


## Error(s)
- `Status: 401 [namespace] [topic]`
- Unauthorized
  - Is your token an admin token?
  - Is your token expired? Tokens expire after 7 days.
  - Did you generate your token for the right host? You need to select the correct instance before generating the token.

- `Status: 403 [namespace] [topic]`
- Forbidden - Your token is valid but is not an admin token
