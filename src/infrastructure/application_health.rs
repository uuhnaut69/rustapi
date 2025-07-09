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

pub struct ApplicationHealth {}

impl ApplicationHealth {
    pub fn new() -> ApplicationHealth {
        ApplicationHealth {}
    }
}

impl Default for ApplicationHealth {
    fn default() -> Self {
        ApplicationHealth::new()
    }
}

#[async_trait::async_trait]
impl HealthRepository for ApplicationHealth {
    async fn health_check(&self) -> Health {
        Health { healthy: true }
    }
}
