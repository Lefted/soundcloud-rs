# V2 Hints

# Configure environment

```json
  "rust-analyzer.runnables.extraEnv": { // for running tests
    "SOUNDCLOUD_CLIENT_ID": "YOUR_CLIENT_ID",
    "SOUNDCLOUD_AUTH_TOKEN": "YOUR_AUTH_TOKEN",
  },
  "rust-analyzer.server.extraEnv": { // for running tests
    "SOUNDCLOUD_CLIENT_ID": "YOUR_CLIENT_ID",
    "SOUNDCLOUD_AUTH_TOKEN": "YOUR_AUTH_TOKEN"
  },
   "rest-client.environmentVariables": { // for using rest-client
    "local": {
      "SOUNDCLOUD_CLIENT_ID": "YOUR_CLIENT_ID",
      "SOUNDCLOUD_AUTH_TOKEN": "YOUR_AUTH_TOKEN"
    }
  }
```

Otherwise, you can set the environment variables in your shell:

```bash
export SOUNDCLOUD_CLIENT_ID=YOUR_CLIENT_ID
export SOUND_CLOUD_AUTH=YOUR_AUTH_TOKEN
```

<details>
<summary>Original README.md</summary>

# soundcloud
[![Build Status](https://travis-ci.org/maxjoehnk/soundcloud-rs.svg?branch=master)](https://travis-ci.org/maxjoehnk/soundcloud-rs)
[![Docs](https://docs.rs/soundcloud/badge.svg)](https://docs.rs/soundcloud)

A Rust library for interacting with the SoundCloud HTTP API.

## Usage

Add the following to your Cargo.toml file.

```toml
[dependencies]
soundcloud = "0.4"
```

To use this crate you need a client id.
Soundcloud currently doesn't allow signup for their api so you need to use an existing client id.

```rust
use soundcloud::Client;

#[tokio::main]
async fn main() {
    let client_id = std::env::var("SOUNDCLOUD_CLIENT_ID").unwrap();
    let client = Client::new(&client_id);
    // ...
}
```

API Usage is documented on [docs.rs](https://docs.rs/soundcloud).

</details>