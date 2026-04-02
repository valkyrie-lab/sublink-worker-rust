//! Sublink Worker - A lightweight subscription converter and manager for proxy protocols
//! 
//! This is a Rust implementation of the sublink-worker project,
//! designed to be compiled for musl-linux-mipsle targets.

mod adapters;
mod builders;
mod config;
mod html;
mod i18n;
mod parsers;
mod runtime;
mod services;
mod utils;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

use crate::config::AppConfig;
use crate::runtime::Runtime;
use crate::services::{ConfigStorageService, ShortLinkService};

/// Application state shared across request handlers
#[derive(Clone)]
pub struct AppState {
    pub runtime: Arc<Runtime>,
    pub short_links: Option<Arc<ShortLinkService>>,
    pub config_storage: Option<Arc<ConfigStorageService>>,
}

/// Root path handler - serves the main HTML page
async fn index_handler(
    Query(params): Query<IndexParams>,
) -> impl IntoResponse {
    let lang = params.lang.unwrap_or_else(|| "zh-CN".to_string());
    Html(html::get_index_html(&lang))
}

#[derive(Debug, Deserialize)]
struct IndexParams {
    lang: Option<String>,
}

/// SingBox config endpoint
async fn singbox_handler(
    Query(params): Query<ConfigParams>,
) -> impl IntoResponse {
    let config_param = params.config.as_deref().unwrap_or("");
    if config_param.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing config parameter").into_response();
    }

    // Parse selected rules
    let selected_rules = parse_selected_rules(params.selected_rules.as_deref());
    let lang = params.lang.unwrap_or_else(|| "zh-CN".to_string());
    let ua = params.ua.as_deref().filter(|s| !s.is_empty());
    let group_by_country = parse_bool(params.group_by_country.as_deref(), false);
    let include_auto_select = parse_bool(params.include_auto_select.as_deref(), true);
    let singbox_version = params.singbox_version.as_deref()
        .or(params.sb_version.as_deref())
        .or(params.sb_ver.as_deref())
        .filter(|s| !s.is_empty());
    let enable_clash_ui = parse_bool(params.enable_clash_ui.as_deref(), false);
    let external_controller = params.external_controller.as_deref().filter(|s| !s.is_empty());
    let external_ui_download_url = params.external_ui_download_url.as_deref().filter(|s| !s.is_empty());

    match crate::builders::SingboxConfigBuilder::new(
        config_param,
        selected_rules,
        lang,
        ua,
        group_by_country,
        include_auto_select,
        singbox_version,
        enable_clash_ui,
        external_controller,
        external_ui_download_url,
    ).await {
        Ok(mut builder) => {
            if let Err(e) = builder.build() {
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("Build failed: {}", e)).into_response();
            }
            Json(builder.get_config().clone()).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, format!("Parse failed: {}", e)).into_response(),
    }
}

/// Clash config endpoint
async fn clash_handler(
    Query(params): Query<ConfigParams>,
) -> impl IntoResponse {
    let config_param = params.config.as_deref().unwrap_or("");
    if config_param.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing config parameter").into_response();
    }

    // Parse selected rules
    let selected_rules = parse_selected_rules(params.selected_rules.as_deref());
    let ua = params.ua.as_deref().filter(|s| !s.is_empty());

    match crate::builders::ClashConfigBuilder::new(config_param, selected_rules, ua).await {
        Ok(mut builder) => {
            if let Err(e) = builder.build() {
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("Build failed: {}", e)).into_response();
            }
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/yaml; charset=utf-8".parse().unwrap());
            (headers, builder.format_config()).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, format!("Parse failed: {}", e)).into_response(),
    }
}

/// Surge config endpoint
async fn surge_handler(
    Query(params): Query<ConfigParams>,
) -> impl IntoResponse {
    let config_param = params.config.as_deref().unwrap_or("");
    if config_param.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing config parameter").into_response();
    }

    // Parse selected rules
    let selected_rules = parse_selected_rules(params.selected_rules.as_deref());
    let ua = params.ua.as_deref().filter(|s| !s.is_empty());

    match crate::builders::SurgeConfigBuilder::new(config_param, selected_rules, ua).await {
        Ok(mut builder) => {
            if let Err(e) = builder.build() {
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("Build failed: {}", e)).into_response();
            }
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "text/plain; charset=utf-8".parse().unwrap());
            (headers, builder.format_config()).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, format!("Parse failed: {}", e)).into_response(),
    }
}

/// Xray endpoint - decode and combine proxy configs
async fn xray_handler(
    Query(params): Query<ConfigParams>,
) -> impl IntoResponse {
    let config_param = params.config.as_deref().unwrap_or("");
    if config_param.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing config parameter").into_response();
    }

    // TODO: Implement Xray config processing
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, config_param.as_bytes());
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/plain; charset=utf-8".parse().unwrap());
    
    (headers, encoded).into_response()
}

/// Subconverter endpoint - generate subconverter config
async fn subconverter_handler(
    Query(_params): Query<ConfigParams>,
) -> impl IntoResponse {
    // TODO: Implement subconverter config generation
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/plain; charset=utf-8".parse().unwrap());
    
    (headers, "# Subconverter config placeholder\n".to_string()).into_response()
}

/// Short link creation endpoint
async fn shorten_handler(
    State(state): State<AppState>,
    Query(params): Query<ShortenParams>,
) -> impl IntoResponse {
    let url = match params.url.as_deref() {
        Some(u) => u,
        None => return (StatusCode::BAD_REQUEST, "Missing URL parameter").into_response(),
    };

    let short_links = match &state.short_links {
        Some(service) => service,
        None => return (StatusCode::SERVICE_UNAVAILABLE, "Short link service unavailable").into_response(),
    };

    match short_links.create_short_link(url, params.short_code.as_deref()).await {
        Ok(code) => (StatusCode::OK, code).into_response(),
        Err(e) => {
            error!("Failed to create short link: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create short link").into_response()
        }
    }
}

/// Config storage endpoint - save configuration
async fn save_config_handler(
    State(state): State<AppState>,
    Json(payload): Json<SaveConfigRequest>,
) -> impl IntoResponse {
    let config_storage = match &state.config_storage {
        Some(service) => service,
        None => return (StatusCode::SERVICE_UNAVAILABLE, "Config storage unavailable").into_response(),
    };

    let content_str = payload.content.to_string();
    match config_storage.save_config(&payload.r#type, &content_str).await {
        Ok(config_id) => (StatusCode::OK, config_id).into_response(),
        Err(e) => {
            error!("Failed to save config: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save config").into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
struct SaveConfigRequest {
    r#type: String,
    content: serde_json::Value,
}

/// Query parameters for config endpoints
#[derive(Debug, Deserialize)]
struct ConfigParams {
    config: Option<String>,
    #[serde(default, alias = "selectedRules")]
    selected_rules: Option<String>,
    #[serde(default, alias = "customRules")]
    custom_rules: Option<String>,
    #[serde(default)]
    ua: Option<String>,
    #[serde(default, alias = "group_by_country")]
    group_by_country: Option<String>,
    #[serde(default, alias = "include_auto_select")]
    include_auto_select: Option<String>,
    #[serde(default, alias = "enable_clash_ui")]
    enable_clash_ui: Option<String>,
    #[serde(default, alias = "external_controller")]
    external_controller: Option<String>,
    #[serde(default, alias = "external_ui_download_url")]
    external_ui_download_url: Option<String>,
    #[serde(default, alias = "config_id")]
    config_id: Option<String>,
    #[serde(default)]
    lang: Option<String>,
    #[serde(default, alias = "singbox_version")]
    singbox_version: Option<String>,
    #[serde(default, alias = "sb_version")]
    sb_version: Option<String>,
    #[serde(default, alias = "sb_ver")]
    sb_ver: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default, alias = "short_code")]
    short_code: Option<String>,
}

/// Query parameters for shorten endpoint
#[derive(Debug, Deserialize)]
struct ShortenParams {
    url: Option<String>,
    short_code: Option<String>,
}

/// Parse selected rules from query parameter
fn parse_selected_rules(raw: Option<&str>) -> Vec<String> {
    if let Some(rules) = raw {
        if rules.is_empty() {
            // Default to minimal rules (same as JS version)
            return vec![
                "Location:CN".to_string(),
                "Private".to_string(),
                "Non-China".to_string(),
            ];
        }

        // Try to parse as JSON array
        if let Ok(parsed) = serde_json::from_str::<Vec<String>>(rules) {
            if parsed.is_empty() {
                // Default to minimal rules (same as JS version)
                return vec![
                    "Location:CN".to_string(),
                    "Private".to_string(),
                    "Non-China".to_string(),
                ];
            }
            return parsed;
        }

        // Otherwise treat as comma-separated list
        return rules.split(',').map(|s| s.trim().to_string()).collect();
    }

    // Default rules
    vec![
        "Location:CN".to_string(),
        "Private".to_string(),
        "Non-China".to_string(),
    ]
}

/// Parse boolean from string
fn parse_bool(value: Option<&str>, fallback: bool) -> bool {
    match value {
        Some(s) => match s.to_lowercase().as_str() {
            "true" | "1" | "on" | "yes" => true,
            "false" | "0" | "off" | "no" => false,
            _ => fallback,
        },
        None => fallback,
    }
}

/// Build the application router
fn build_router(state: AppState) -> Router {
    Router::new()
        // API routes
        .route("/singbox", get(singbox_handler))
        .route("/clash", get(clash_handler))
        .route("/surge", get(surge_handler))
        .route("/xray", get(xray_handler))
        .route("/subconverter", get(subconverter_handler))
        .route("/shorten-v2", get(shorten_handler))
        .route("/config", post(save_config_handler))
        // Static files - serve index.html for root
        .route("/", get(index_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("sublink_worker=info".parse()?)
        )
        .init();

    info!("Starting Sublink Worker (Rust version)");

    // Load configuration
    let config = AppConfig::load().map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;
    
    // Create runtime
    let runtime = Runtime::new(config.clone()).await?;
    
    // Initialize services
    let short_links = runtime.redis.as_ref().map(|redis| {
        Arc::new(ShortLinkService::new(redis.clone(), config.short_link_ttl_seconds))
    });

    let config_storage = runtime.sqlite.as_ref().map(|sqlite| {
        Arc::new(ConfigStorageService::new(sqlite.clone(), config.config_ttl_seconds))
    });

    // Create application state
    let state = AppState {
        runtime: Arc::new(runtime),
        short_links,
        config_storage,
    };

    // Build router
    let app = build_router(state);

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
