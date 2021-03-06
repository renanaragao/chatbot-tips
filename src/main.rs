#[macro_use]
extern crate rocket;

mod database;
mod models;
mod repository;
mod telegram;

use database::MongoDB;
use dilib::{add_scoped_trait, global::init_container, resolve};
use repository::user::{IUserRepository, UserRepository};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use telegram::models::Update;
use uri_builder::URI;

#[post("/update", data = "<update>")]
async fn new_update(database: &State<MongoDB>, update: Json<Update>) -> Status {
    let resolve = resolve!(trait IUserRepository).unwrap();
    let repository = resolve.as_ref();

    let mut user = models::user::User {
        id: update.message.from.id,
        first_name: update.message.from.first_name.to_string(),
        last_name: update.message.from.last_name.to_string(),
        language_code: update.message.from.language_code.to_string(),
        is_bot: update.message.from.is_bot,
    };

    repository.save(&database.db, &mut user).await.unwrap();

    Status::Accepted
}

#[launch]
async fn rocket() -> _ {
    let token = std::env::var("TELEGRAM_TOKEN")
        .or(Err("TELEGRAM_TOKEN environment variable missing"))
        .unwrap();
    let path = format!("bot{token}", token = token).to_string();
    let _uri = URI::new("https").host("api.telegram.org").path(&path);

    init_container(|container| {
        add_scoped_trait!(container, IUserRepository => UserRepository()).unwrap();
    })
    .expect("unable to initialize the container");

    rocket::build()
        .attach(database::init())
        .mount("/", routes![new_update])
}

#[cfg(test)]
async fn rocket_test() -> ::rocket::Rocket<::rocket::Build> {
    rocket::build()
        .attach(database::init())
        .mount("/", routes![new_update])
}

#[cfg(test)]
mod test {
    use crate::{repository::user::{IUserRepository, UserRepositoryFake}, rocket_test};
    use dilib::{add_scoped_trait, global::init_container};

    use super::rocket;
    use rocket::http::{ContentType, Status};

    #[rocket::async_test]
    async fn should_save_user() -> Result<(), ()> {
        use rocket::local::asynchronous::Client;

        let client = Client::tracked(rocket_test().await).await.unwrap();

        init_container(|container| {
            add_scoped_trait!(container, IUserRepository => UserRepositoryFake()).unwrap();
        })
        .unwrap();

        let req = client.post("/update").header(ContentType::JSON).body(
            r##"{
                "update_id": 500612411,
                "message": {
                    "message_id": 74,
                    "from": {
                        "id": 638061488,
                        "is_bot": false,
                        "first_name": "Renan",
                        "last_name": "Arag??o",
                        "language_code": "en"
                    },
                    "chat": {
                        "id": 638061488,
                        "first_name": "Renan",
                        "last_name": "Arag??o",
                        "type": "private"
                    },
                    "date": 1653613931,
                    "text": "teste"
                }
            }"##,
        );

        let response = req.dispatch().await;

        assert_eq!(response.status(), Status::Accepted);

        Ok(())
    }
}
