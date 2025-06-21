# 🎶 Spot API - Unofficial Gaana API

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/axum-web%20framework-blue?style=for-the-badge)](https://github.com/tokio-rs/axum)
[![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)

A fast, reliable, and comprehensive JSON API for accessing Gaana music data. Built with Rust and Axum for high performance and safety.

## 🚀 Features

- **Fast & Reliable**: Built with Rust for memory safety and performance
- **Comprehensive**: Search songs, albums, artists, get trending content, new releases, and charts
- **RESTful API**: Clean, intuitive endpoints with proper HTTP status codes
- **Auto-Generated Docs**: Interactive API documentation with Swagger UI
- **Stream URLs**: Decrypt and provide multiple quality stream URLs
- **CORS Support**: Ready for web applications
- **Docker Ready**: Easy deployment with Docker and docker-compose

## 📋 API Endpoints

### Base URL: `http://localhost:8000`

| Endpoint | Method | Description | Example |
|----------|--------|-------------|---------|
| `/` | GET | API information and documentation link | `/` |
| `/docs` | GET | Interactive API documentation | `/docs` |
| `/songs/search` | GET | Search songs by name | `/songs/search?query=tyler%20herro&limit=5` |
| `/songs/info` | GET | Get song details by SEO key | `/songs/info?seokey=tyler-herro` |
| `/albums/search` | GET | Search albums by name | `/albums/search?query=all%20over%20the%20place` |
| `/albums/info` | GET | Get album details by SEO key | `/albums/info?seokey=tyler-herro` |
| `/artists/search` | GET | Search artists by name | `/artists/search?query=KSI&limit=5` |
| `/artists/info` | GET | Get artist details by SEO key | `/artists/info?seokey=jack-harlow` |
| `/playlists/info` | GET | Get playlist details by SEO key | `/playlists/info?seokey=gaana-dj-gaana-international-top-50` |
| `/trending` | GET | Get trending songs by language | `/trending?lang=English&limit=20` |
| `/newreleases` | GET | Get new releases by language | `/newreleases?lang=English&limit=15` |
| `/charts` | GET | Get top charts (popular playlists) | `/charts?limit=25` |

## 🛠️ Installation & Setup

### Prerequisites

- [Rust](https://rustup.rs/) (1.87 +)
- [Docker](https://www.docker.com/) (optional, for containerized deployment)

### Method 1: Local Development

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/spot-server-v2.git
   cd spot-server-v2
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Create environment file**
   ```bash
   cp .env.example .env
   ```

4. **Run the server**
   ```bash
   cargo run
   ```

5. **Access the API**
   - API: http://localhost:8000
   - Documentation: http://localhost:8000/docs

### Method 2: Docker

1. **Using Docker Compose**
   ```bash
   # Basic deployment
   docker-compose up -d
   
   # With nginx reverse proxy
   docker-compose --profile with-nginx up -d
   ```

2. **Using Docker directly**
   ```bash
   # Build the image
   docker build -t spot-server .
   
   # Run the container
   docker run -p 8000:8000 -e PORT=8000 spot-server
   ```

## 📊 Response Examples

### Song Search Response
```json
[
  {
    "seokey": "tyler-herro",
    "album_seokey": "tyler-herro",
    "track_id": "32408795",
    "title": "Tyler Herro",
    "artists": "Jack Harlow",
    "artist_seokeys": "jack-harlow",
    "artist_ids": "123456",
    "artist_image": "https://a10.gaanacdn.com/gn_img/artists/XYybzrb2gz/Yybzn4Bgb2/size_m_1607927137.webp",
    "album": "Tyler Herro",
    "album_id": "987654",
    "duration": "02:36",
    "popularity": "8.5",
    "genres": "Hip Hop",
    "is_explicit": 1,
    "language": "English",
    "label": "Generation Now/Atlantic",
    "release_date": "2020-10-22",
    "play_count": "<100K",
    "favorite_count": 202,
    "song_url": "https://gaana.com/song/tyler-herro",
    "album_url": "https://gaana.com/album/tyler-herro",
    "images": {
      "urls": {
        "large_artwork": "https://a10.gaanacdn.com/gn_img/albums/4Z9bqo3yQn/Z9bq2AG1Ky/size_l.jpg",
        "medium_artwork": "https://a10.gaanacdn.com/gn_img/albums/4Z9bqo3yQn/Z9bq2AG1Ky/size_m.jpg",
        "small_artwork": "https://a10.gaanacdn.com/gn_img/albums/4Z9bqo3yQn/Z9bq2AG1Ky/size_s.jpg"
      }
    },
    "stream_urls": {
      "urls": {
        "very_high_quality": "https://stream-cdn.gaana.com/.../320.mp4.xvod/master.m3u8",
        "high_quality": "https://stream-cdn.gaana.com/.../128.mp4.xvod/master.m3u8",
        "medium_quality": "https://stream-cdn.gaana.com/.../64.mp4.xvod/master.m3u8",
        "low_quality": "https://stream-cdn.gaana.com/.../16.mp4.xvod/master.m3u8"
      }
    }
  }
]
```

### Error Response
```json
{
  "error": "No results found",
  "message": "No search results for the given query: invalid-song"
}
```

## 🔧 Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `8000` | Server port |
| `RUST_LOG` | `spot_server_v2=debug` | Logging level |
| `CORS_ALLOW_ORIGINS` | `*` | CORS allowed origins |

### Supported Languages

- English
- Hindi (default)
- Punjabi
- Telugu
- Tamil
- Bengali
- Gujarati
- Kannada
- Malayalam
- Marathi
- Odia
- Assamese

## 🏗️ Architecture

```
src/
├── main.rs              # Application entry point
├── api/                 # API endpoint handlers
│   ├── mod.rs
│   ├── base.rs          # Base API functionality
│   ├── songs.rs         # Song endpoints
│   ├── albums.rs        # Album endpoints
│   ├── artists.rs       # Artist endpoints
│   ├── playlists.rs     # Playlist endpoints
│   ├── trending.rs      # Trending endpoints
│   ├── newreleases.rs   # New releases endpoints
│   └── charts.rs        # Charts endpoints
├── models/              # Data structures
│   ├── mod.rs
│   ├── song.rs          # Song models
│   ├── album.rs         # Album models
│   ├── artist.rs        # Artist models
│   ├── playlist.rs      # Playlist models
│   ├── images.rs        # Image URL models
│   ├── stream_urls.rs   # Stream URL models
│   └── error.rs         # Error models
└── utils/               # Utility functions
    ├── mod.rs
    ├── encryption.rs    # Stream URL decryption
    └── formatting.rs    # Data formatting helpers
```

## 🧪 Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## 📈 Performance

- **Async/Await**: Non-blocking I/O for high concurrency
- **Connection Pooling**: Efficient HTTP client management
- **Memory Safety**: Rust's ownership system prevents memory leaks
- **Zero-Copy**: Efficient string and data handling

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This is an unofficial API for educational purposes only. It is not affiliated with Gaana.com. Please respect their terms of service and rate limits.

## 🔗 Links

- [Gaana.com](https://gaana.com) - Official Gaana website
- [Axum Framework](https://github.com/tokio-rs/axum) - Web framework
- [Rust Language](https://www.rust-lang.org/) - Programming language

---

Made with ❤️ and 🦀 Rust
