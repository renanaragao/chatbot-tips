#[macro_use] extern crate rocket;

mod models;
mod repository;
mod telegram;

use rocket::http::Status;
use rocket::serde::json::Json;
use uri_builder::URI;
use telegram::models::Update as Update;


#[post("/update", data = "<update>")]
fn new_update(update: Json<Update>) -> Status {
    println!("{:?}", update);
    Status::Accepted
}

#[launch]
fn rocket() -> _ {

    let token = std::env::var("TELEGRAM_TOKEN").or(Err("TELEGRAM_TOKEN environment variable missing")).unwrap();
    let path = format!("bot{token}", token = token).to_string();
    let _uri = URI::new("https").host("api.telegram.org").path(&path);

    rocket::build().mount("/", routes![new_update])
}
