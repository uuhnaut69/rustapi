/*
 * Copyright 2025 uuhnaut69
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::infrastructure::container::Container;
use crate::infrastructure::http::health_handler::health_check;
use axum::Router;
use axum::http::Method;
use axum::routing::get;
use std::env;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::decompression::RequestDecompressionLayer;

pub async fn initialize_server() {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Invalid PORT environment variable");
    let ioc_container = Arc::new(Container::default());

    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(ioc_container.clone())
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ]),
        );

    let listener = TcpListener::bind(format!("{}:{}", Ipv4Addr::LOCALHOST, port))
        .await
        .unwrap();

    tracing::info!(
        "🚀 Server listening on {}",
        format!("{}:{}", Ipv4Addr::LOCALHOST, port)
    );

    axum::serve(listener, app).await.unwrap();
}
