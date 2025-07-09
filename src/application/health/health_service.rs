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
use crate::application::health::health_repository::HealthRepository;
use crate::domain::health::Health;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait HealthService: Send + Sync + 'static {
    async fn health_check(&self) -> Health;
}

pub struct HealthServiceImpl {
    pub health_repository: Arc<dyn HealthRepository>,
}

impl HealthServiceImpl {
    pub fn new(health_repository: Arc<dyn HealthRepository>) -> Self {
        HealthServiceImpl { health_repository }
    }
}

#[async_trait::async_trait]
impl HealthService for HealthServiceImpl {
    async fn health_check(&self) -> Health {
        self.health_repository.health_check().await
    }
}
