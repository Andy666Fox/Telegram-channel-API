# TG Parser API
### A Rust-based microservice for extracting metadata from public Telegram channels (subscriber counts, media counts, and content types) via the t.me preview interface.
## Features
- ### Scraping of Telegram's web preview pages.
- ### In-memory caching (Moka cache).
- ### Rate limiting per IP (Tower-governor).
- ### Structured logging and tracing.
- ### Automatically generated OpenAPI/Swagger documentation.

## Quick Start (Docker)
### Build and run using Docker Compose:
`docker-compose up -d --build`

## API Endpoints
- ### `GET /get_info?name={username} — Retrieve channel data.`

- ### `GET /swagger-ui — Interactive API documentation.`

- ### `GET /api-docs/openapi.json — OpenAPI specification file.`