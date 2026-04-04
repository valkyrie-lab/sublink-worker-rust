# Sublink Worker (Rust Version)

> A lightweight subscription converter, rewritten in Rust. **Powered by Qwen 3.5 & MiniMax M2.7**.

## ✨ AI Contributions

- **Qwen 3.5**
- **MiniMax M2.7**

## Features

- 🚀 High performance, low memory footprint
- 🔒 Memory safety with Rust
- 📦 Single binary deployment with musl static linking
- 🎯 Cross-platform (mipsel/aarch64/x86_64)
- 🌐 Multi-language support

## Supported Protocols

ShadowSocks · VMess · VLESS · Hysteria2 · Trojan · TUIC

## Client Support

Sing-Box · Clash · Surge · Xray/V2Ray

## Quick Start

### Build from Source

```bash
cargo build --target <target> --release
```

For mipsel-linux-musl target

```bash
nix-shell --run "cargo build --target mipsel-unknown-linux-musl --release -Zbuild-std=std,panic_abort"
```

### OpenWRT Packaging

```bash
# Build ipk package
nix-shell package.nix
build-ipk

# Output: sublink_1.0.0-1_mipsel_24kc.ipk
```

Install on router:
```bash
opkg install sublink_1.0.0-1_mipsel_24kec.ipk
```

### Configuration

Create `config.toml`:

```toml
host = "0.0.0.0"
port = 8787
database_path = "/data/sublink.db"
log_level = "info"
```

Or configure via environment variables.

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

## License

MIT License

## Disclaimer

This project is for learning purposes only. Do not use for illegal purposes.
