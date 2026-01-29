# OAR API

This crate contains the web API layer for OAR.

## What it does

The API crate provides:
- HTTP endpoints for user registration and login
- RESTful API routes using Axum framework
- Request/response handling and validation
- Integration with the domain and infrastructure layers



## Contributing

To add new API endpoints:

1. Create handler functions in `src/handlers/`
2. Add route definitions in `src/handlers/mod.rs`
3. Update request/response DTOs in `src/handlers/users/dtos.rs` if needed

## Running the API

```bash
cd crates/oar-api
cargo run
```

The server will start on http://localhost:3000