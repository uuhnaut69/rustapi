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
use crate::application::health::health_service::{HealthService, HealthServiceImpl};
use crate::infrastructure::application_health::ApplicationHealth;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub health_service: Arc<dyn HealthService>,
}

impl AppState {
    pub fn new() -> Self {
        let application_health = Arc::new(ApplicationHealth::default());
        let health_service = Arc::new(HealthServiceImpl {
            health_repository: application_health.clone(),
        });
        AppState { health_service }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
