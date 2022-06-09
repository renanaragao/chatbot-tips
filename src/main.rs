#[macro_use] extern crate rocket;

mod models;
mod repository;
mod telegram;

use uri_builder::URI;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {

    let token = std::env::var("TELEGRAM_TOKEN").or(Err("TELEGRAM_TOKEN environment variable missing")).unwrap();
    let path = format!("bot{token}", token = token).to_string();
    let _uri = URI::new("https").host("api.telegram.org").path(&path);

    rocket::build().mount("/", routes![index])
}
