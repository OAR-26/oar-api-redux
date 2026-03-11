# OAR Rust API

A modern, high-performance REST API built with Rust for the OAR project.

## 📦 Project Structure

```
oar-api-redux/
├── crates/
│   ├── oar-api/           # Main API server
│   ├── oar-domain/        # Domain models and business logic
│   └── oar-infrastructure/ # Database and external service integrations
├── README.md
└── Cargo.toml
```

### Core Crates

- **`oar-api`**: REST API server using Axum-web framework
- **`oar-domain`**: Domain models, entities, and business logic
- **`oar-infrastructure`**: Database repositories and external service adapters

## Quick Start

### Prerequisites

- Rust 1.75+ installed
- Environment variables configured

### Installation

1. Clone the repository:

```bash
git clone https://github.com/OAR-26/oar-api-redux.git
cd oar-api-redux
```

2. Install dependencies:

```bash
cargo build
```

3. Set up environment variables:

```bash
cp .env.example .env
# Edit .env with your database credentials
```

4. Run the server:

```bash
cargo run --bin oar-api
```

### Development

Start in development mode with auto-reload:

```bash
cargo watch -x run
```
