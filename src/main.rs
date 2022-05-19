use reqwest::Result;
mod telegram;

use telegram::updates as updates;

#[tokio::main]
async fn main() -> Result<()> {

    let updates = updates::get("https://api.telegram.org".to_string(),"token".to_string()).await?;

    println!("{:?}", updates);

    Ok(())
}
