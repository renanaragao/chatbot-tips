use reqwest::Result;
mod telegram;

use telegram::updates as updates;

#[tokio::main]
async fn main() -> Result<()> {

    let updates = updates::get("https://api.telegram.org","token").await?;

    println!("{:?}", updates);

    Ok(())
}
