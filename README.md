# Auth Extension for CoreX-API

**Auth Extension** is a plugin for the [CoreX-API](https://crates.io/crates/corex-api) framework. It provides an authentication endpoint for your modular API system.

## Installation

Add `auth-extension` to your `Cargo.toml`:

```toml
[dependencies]
auth-extension = { git = "https://github.com/denver-code/auth-extension" }
tokio = { version = "1", features = ["full"] }
corex-api = "0.1.1"
```

## Usage

1. Register the `AuthExtension` with your CoreX-API system:

```rust
use auth_extension::AuthExtension;
use corex_api::CoreX;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let host = "0.0.0.0".to_string();
    let port = 3000;
    let mut core = CoreX::new(host, port);

    core.register_extension(Arc::new(AuthExtension));

    core.run().await;
}
```

2. Start the server and test the `/auth` endpoint:

```bash
curl http://localhost:3000/auth
```

Response:

```json
{
  "message": "Auth endpoint"
}
```

## License

This project is licensed under:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))


## Contributing

Contributions are welcome! If you'd like to contribute, please:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Submit a pull request.
