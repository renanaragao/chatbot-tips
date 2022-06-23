#[macro_use]
extern crate rocket;

mod database;
mod models;
mod repository;
mod telegram;

use rocket::State;
use dilib::{global::init_container, resolve};
use repository::user::UserRepositoryFactory;
use rocket::http::Status;
use rocket::serde::json::Json;
use telegram::models::Update;
use uri_builder::URI;
use database::MongoDB;

#[post("/update", data = "<update>")]
async fn new_update(database: &State<MongoDB>, update: Json<Update>) -> Status {
    let resolve = resolve!(UserRepositoryFactory).unwrap();
    let factory = resolve.as_ref();

    let mut changed_user = models::user::User {
        id: 234,
        first_name: String::from("Renan Aragão"),
        last_name: String::from("Ferreira"),
        language_code: String::from("en"),
        is_bot: true,
    };

    let repository = factory.create(database.client.clone());

    repository.save(&mut changed_user).await.unwrap();

    // let rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(async {
    //     user_repository.save(&mut changed_user).await.unwrap();
    // });

    println!("{:?}", update);
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
        container.add_scoped(|| UserRepositoryFactory).unwrap();
    }).expect("unable to initialize the container");

    rocket::build()
        .attach(database::init())
        .mount("/", routes![new_update])
}

