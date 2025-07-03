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
use crate::infrastructure::app_state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

const HEALTH_TAG: &str = "Health";

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct HealthResponse {
    healthy: bool,
}

#[utoipa::path(
    tag = HEALTH_TAG,
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse)
    )
)]
pub async fn health_check(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let result = app_state.health_service.health_check().await;
    Json(HealthResponse {
        healthy: result.healthy,
    })
}
