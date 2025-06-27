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
use dotenvy::dotenv;
use rustapi::infrastructure::observability::init_observability;
use rustapi::infrastructure::server::initialize_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    init_observability();
    initialize_server().await;

    Ok(())
}
