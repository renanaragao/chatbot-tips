/*
 * Copyright 2021 Nabil Hachicha.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::Client;
use rocket::fairing::AdHoc;

#[derive(Debug)]
pub struct MongoDB {
    pub client: Client,
}

impl MongoDB {
    fn new(client: Client) -> Self {
        MongoDB { client }
    }
}

pub fn init() -> AdHoc {
    AdHoc::on_ignite("Connect to MongoDB cluster", |rocket| async {
        match connect().await {
            Ok(client) => rocket.manage(MongoDB::new(client)),
            Err(error) => {
                panic!("Cannot connect to MDB instance:: {:?}", error)
            }
        }
    })
}

async fn connect() -> mongodb::error::Result<Client> {
    let mdb_uri = std::env::var("MDB_URL")
        .or(Err("MDB_URL environment variable missing"))
        .unwrap();
    let client_options = ClientOptions::parse(mdb_uri).await?;
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("chat-tip")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    Ok(client)
}
