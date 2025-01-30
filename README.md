# Auth Extension for CoreX-API with SQLite

## Usage   
Add dependency to your `Cargo.toml` file
```toml
auth-extension-sqlite = { git = "https://github.com/denver-code/auth-extension-sqlite" }
```

Create a new instance of the extension:
```rust
let auth_extension = create_auth_extension()
      .await
      .expect("Failed to create auth extension");
```
And then register it with the CoreX instance:
```rust
core.register_extension(auth_extension);
```

This method slightly differs from the other extensions as it requires an async function to create the extension at first and then register it with the CoreX instance.  
Full example:
```rust
use auth_extension_sqlite::create_auth_extension;
use corex_api::CoreX;

#[tokio::main]
async fn main() {
    let host = "0.0.0.0".to_string();
    let port = 3000;
    let mut core = CoreX::new(host, port);

    let auth_extension = create_auth_extension()
        .await
        .expect("Failed to create auth extension");

    core.register_extension(auth_extension);

    core.run().await;
}

```

## API 
The extension provides the following API:
- POST `/register` - Register a new user
  ```json
  {
    "username": "username",
    "password": "password"
  }
  ```
- POST `/login` - Login a user
  ```json
  {
    "username": "username",
    "password": "password"
  }
  ```