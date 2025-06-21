# Copilot Instructions for Spot Server

<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

## Project Overview
This is a Rust-based web API server using the Axum framework that provides an unofficial JSON API for Gaana.com (Indian Music Streaming Service). The API provides comprehensive music data access through RESTful endpoints.

## Technical Stack
- **Language**: Rust
- **Web Framework**: Axum (async web framework)
- **HTTP Client**: reqwest (for external API calls)
- **Serialization**: serde (JSON handling)
- **Documentation**: utoipa (OpenAPI/Swagger)
- **Encryption**: aes, cbc (for stream URL decryption)
- **Deployment**: Docker

## Architecture Guidelines

### Code Organization
- `src/main.rs` - Application entry point and route definitions
- `src/api/` - API endpoint handlers (songs, albums, artists, etc.)
- `src/models/` - Data structures and response models
- `src/utils/` - Utility functions (encryption, formatting)

### API Design Patterns
- Use async/await for all HTTP operations
- Implement proper error handling with custom error types
- Return structured JSON responses with consistent formats
- Use query parameters for filtering and pagination
- Follow RESTful conventions

### Error Handling
- Always return meaningful error messages
- Use proper HTTP status codes (200, 404, 500)
- Implement custom error types in `models/error.rs`
- Handle network failures gracefully

### Data Processing
- Process Gaana API responses in `api/base.rs`
- Extract and format artist information properly
- Handle optional fields with `Option<T>`
- Decrypt stream URLs using AES encryption

### Dependencies Management
- Use specific versions in Cargo.toml
- Group related dependencies logically
- Include feature flags where needed
- Keep dependencies up to date

## API Endpoints
The server provides these main endpoints:
- Song search and info: `/songs/search`, `/songs/info`
- Album search and info: `/albums/search`, `/albums/info`
- Artist search and info: `/artists/search`, `/artists/info`
- Playlist info: `/playlists/info`
- Trending content: `/trending`
- New releases: `/newreleases`
- Charts: `/charts`

## Development Guidelines
- Use `#[derive(Debug, Serialize, Deserialize)]` for data structures
- Add `ToSchema` for OpenAPI documentation
- Implement proper query parameter validation
- Use URL encoding for API requests
- Follow Rust naming conventions (snake_case)

## Testing and Deployment
- Build with `cargo build --release`
- Run with `cargo run`
- Use Docker for containerized deployment
- Environment variables for configuration
