mod models;
mod repository;
mod telegram;

use uri_builder::URI;

#[tokio::main]
async fn main() {
    let token = std::env::var("TELEGRAM_TOKEN").or(Err("TELEGRAM_TOKEN environment variable missing")).unwrap();
    let path = format!("bot{token}", token = token).to_string();

    telegram::send_message::post(
        URI::new("https").host("api.telegram.org").path(&path),
        638061488,
        "asdasd",
    )
    .await
    .unwrap();

    println!("Finished");
}
