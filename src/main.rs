#[macro_use]
extern crate rocket;

mod database;
mod models;
mod repository;
mod services;
mod telegram;

use database::MongoDB;
use dilib::{add_scoped_trait, global::init_container, resolve};
use repository::user::{IUserRepository, UserRepository};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use services::chatbot::{ChatbotService, IChatbotService};
use telegram::models::{Update, UriTelegram};
use telegram::telegram::{ITelegramService, TelegramService};
use uri_builder::URI;

struct StateUriTelegram {
    uri: UriTelegram,
}

#[post("/update", data = "<update>")]
async fn new_update(
    database: &State<MongoDB>,
    uri_telegram: &State<StateUriTelegram>,
    update: Json<Update>,
) -> Status {
    let resolve_repository = resolve!(trait IUserRepository).unwrap();
    let repository = resolve_repository.as_ref();

    let mut user = models::user::User {
        id: update.message.from.id,
        first_name: update.message.from.first_name.to_string(),
        last_name: update.message.from.last_name.to_string(),
        language_code: update.message.from.language_code.to_string(),
        is_bot: update.message.from.is_bot,
    };

    repository.save(&database.db, &mut user).await.unwrap();

    let resolve_chatbot = resolve!(trait IChatbotService).unwrap();
    let service = resolve_chatbot.as_ref();

    service
        .new_update(&uri_telegram.uri, update.into_inner())
        .await
        .unwrap();

    Status::Accepted
}

#[get("/health")]
fn _health() -> Status {
    Status::Ok
}

#[launch]
async fn rocket() -> _ {
    init_container(|container| {
        add_scoped_trait!(container, IUserRepository => UserRepository()).unwrap();
        add_scoped_trait!(container, ITelegramService => TelegramService()).unwrap();
        add_scoped_trait!(container, IChatbotService => ChatbotService()).unwrap();
    })
    .expect("unable to initialize the container");

    rocket::build()
        .attach(database::init())
        .manage(StateUriTelegram {
            uri: UriTelegram::new(
                "api.telegram.org".to_string(),
                std::env::var("TELEGRAM_TOKEN")
                    .or(Err("TELEGRAM_TOKEN environment variable missing"))
                    .unwrap()
                    .to_string(),
                "https".to_string(),
                443,
            ),
        })
        .mount("/", routes![new_update, _health])
}

#[cfg(test)]
async fn rocket_test() -> ::rocket::Rocket<::rocket::Build> {
    rocket::build()
        .attach(database::init())
        .manage(StateUriTelegram {
            uri: UriTelegram::new(
                "api.telegram.org".to_string(),
                "123456789:ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
                "https".to_string(),
                443,
            ),
        })
        .mount("/", routes![new_update])
}

#[cfg(test)]
mod test {
    use crate::{
        repository::user::{IUserRepository, UserRepositoryFake},
        rocket_test,
        services::chatbot::{ChatbotServiceFake, IChatbotService},
    };
    use dilib::{add_scoped_trait, global::init_container};

    use super::rocket;
    use rocket::http::{ContentType, Status};

    #[rocket::async_test]
    async fn should_save_user() -> Result<(), ()> {
        use rocket::local::asynchronous::Client;

        let client = Client::tracked(rocket_test().await).await.unwrap();

        init_container(|container| {
            add_scoped_trait!(container, IUserRepository => UserRepositoryFake()).unwrap();
            add_scoped_trait!(container, IChatbotService => ChatbotServiceFake()).unwrap();
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
                        "last_name": "Aragão",
                        "language_code": "en"
                    },
                    "chat": {
                        "id": 638061488,
                        "first_name": "Renan",
                        "last_name": "Aragão",
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
