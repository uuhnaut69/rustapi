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
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    healthy: bool,
}

pub async fn health_check(State(container): State<Arc<Container>>) -> impl IntoResponse {
    let result = container.health_service.health_check().await;
    Json(HealthResponse {
        healthy: result.healthy,
    })
}
