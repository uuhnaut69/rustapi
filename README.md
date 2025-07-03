# RustAPI

A modern, high-performance REST API template built with Rust using Axum framework and clean architecture principles.

## 🚀 Features

- **High Performance**: Rust + Axum for speed and memory efficiency
- **Clean Architecture**: Domain, application, and infrastructure layers
- **OpenAPI Documentation**: Auto-generated docs with Swagger UI and Scalar
- **Observability**: Health checks, structured logging, and tracing
- **Production Ready**: CORS, compression, timeouts, Docker support
- **Development Tools**: Pre-configured linting, formatting, and testing

## 🏗️ Architecture

```
src/
├── domain/           # Business logic and entities
├── application/      # Use cases and business rules  
├── infrastructure/   # External concerns (HTTP, DB, etc.)
├── lib.rs
└── main.rs
```

## 🛠️ Quick Start

1. **Clone and setup**
   ```bash
   git clone <repository-url> your-project-name
   cd your-project-name
   rm -rf .git && git init
   ```

2. **Update project details in `Cargo.toml`**

3. **Run the application**
   ```bash
   # Development
   cargo run
   
   # With Docker (includes Redis + PostgreSQL)
   docker-compose up -d
   ```

The API will be available at `http://localhost:3000`

## 📖 API Documentation

- **Swagger UI**: http://localhost:3000/swagger-ui
- **Scalar UI**: http://localhost:3000/scalar
- **OpenAPI JSON**: http://localhost:3000/api-docs/openapi.json

### Endpoints

- **GET** `/health` - Health check endpoint

## 🔧 Development

```bash
# Testing
cargo test

# Formatting & Linting
cargo fmt
cargo clippy

# Coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

## 🐳 Docker Services

The `compose.yaml` includes:

- **Redis**: Port 6379 (UI: 8001)
- **PostgreSQL**: Port 5432 (postgres/postgres)

```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop and cleanup
docker-compose down -v
```

## ⚙️ Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Server port | `3000` |
| `RUST_LOG` | Logging level | `info` |

## 🎯 Next Steps

1. Add domain models in `src/domain/`
2. Implement business logic in `src/application/`
3. Create HTTP handlers in `src/infrastructure/http/`
4. Add database repositories and integrations
5. Extend OpenAPI documentation
6. Configure authentication and authorization

## 📝 License

Licensed under the Apache License 2.0 - see [LICENSE](LICENSE) file for details.
