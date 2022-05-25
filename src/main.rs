mod models;
mod repository;
mod telegram;

use mongodb::{options::ClientOptions, Client, Database};
use models::user::User as User;
use repository::users::Users as Users;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("TELEGRAM_TOKEN").or(Err("TELEGRAM_TOKEN environment variable missing"))?;

    let updates = telegram::updates::get(
        "https://api.telegram.org",
        &token,
    )
    .await?;

    match updates.first() {
        Some(update) => {
            let from = &update.message.from;

            let mut user = User {
                id: from.id,
                first_name: String::from(&from.first_name),
                last_name: String::from(&from.last_name),
                language_code: String::from(&from.language_code),
                is_bot: from.is_bot,
            };
        
            Users::new(get_db().await?).save(&mut user).await?;
        }
        None => println!("No updates"),
    }

    println!("Finished");

    Ok(())
}

async fn get_db() -> Result<Database, mongodb::error::Error> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    let client = Client::with_options(client_options)?;

    Ok(client.database("chat-tip"))
}
