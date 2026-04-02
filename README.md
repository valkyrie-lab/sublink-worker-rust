# Sublink Worker (Rust Version)

> A lightweight subscription converter and manager for proxy protocols, rewritten in Rust.

## Features

- 🚀 High performance with minimal resource usage
- 🔒 Memory safety with Rust
- 📦 Single binary deployment with musl libc
- 🎯 Cross-compiled for mipsel-unknown-linux-musl (MIPS LE architecture)
- 🌐 Multi-language support (Chinese, English, Persian, Russian)

## Supported Protocols

- ShadowSocks
- VMess
- VLESS
- Hysteria2
- Trojan
- TUIC

## Client Support

- Sing-Box
- Clash
- Surge
- Xray/V2Ray

## Quick Start

### Build from Source

```bash
# Install mipsel cross compiler
# Ubuntu/Debian
apt-get install gcc-mipsel-linux-gnu

# Arch Linux
pacman -S mipsel-linux-gnu-gcc

# Build
./scripts/build-mipsel.sh
```

### Using Docker

```bash
docker build -t sublink-worker-rust .
docker run -p 8787:8787 sublink-worker-rust
```

### Configuration

Create a `config.toml` file:

```toml
host = "0.0.0.0"
port = 8787
redis_url = "redis://localhost:6379"
database_path = "/data/sublink.db"
short_link_ttl_seconds = 3600
config_ttl_seconds = 86400
log_level = "info"
```

Or use environment variables:

```bash
export HOST=0.0.0.0
export PORT=8787
export REDIS_URL=redis://localhost:6379
export DATABASE_PATH=/data/sublink.db
export SHORT_LINK_TTL=3600
export CONFIG_TTL=86400
export LOG_LEVEL=info
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Web interface |
| `/singbox` | GET | Generate Sing-Box config |
| `/clash` | GET | Generate Clash config |
| `/surge` | GET | Generate Surge config |
| `/xray` | GET | Generate Xray config |
| `/subconverter` | GET | Generate subconverter config |
| `/shorten-v2` | GET | Create short link |
| `/b/:code` | GET | Redirect to Sing-Box config |
| `/c/:code` | GET | Redirect to Clash config |
| `/s/:code` | GET | Redirect to Surge config |
| `/x/:code` | GET | Redirect to Xray config |
| `/config` | POST | Save configuration |
| `/resolve` | GET | Resolve short link |

## Query Parameters

### `/singbox`, `/clash`, `/surge`, `/xray`

- `config` (required): Subscription URL or Base64 config
- `selected_rules`: Rule preset name or JSON array
- `custom_rules`: Custom rules as JSON array
- `ua`: User-Agent for fetching subscriptions
- `group_by_country`: Group proxies by country
- `include_auto_select`: Include auto-select proxy group
- `config_id`: Use stored configuration
- `lang`: Language preference

## Building for Other Targets

```bash
# Add target
rustup target add <target-triple>

# Build
cargo build --release --target <target-triple>
```

Common targets:
- `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-musl`
- `armv7-unknown-linux-musleabihf`
- `mipsel-unknown-linux-musl`

## License

MIT License

## Disclaimer

This project is for learning and exchange purposes only. Please do not use it for illegal purposes.
