use reqwest::Error;

use super::models::{Response, Update};

pub async fn get(host: String, token: String) -> Result<Vec<Update>, Error> {
    let request_url = format!(
        "{host}/bot{token}/getUpdates",
        token = token.to_string(),
        host = host.to_string()
    );

    let response = reqwest::get(&request_url).await?;

    let result = response.json::<Response>().await?;

    Ok(result.result)
}

#[cfg(test)]
mod tests {

    use reqwest::Error;
use mockito;
    use mockito::mock;

    #[tokio::test]
    async fn get_updates() -> Result<(), Error> {
        let host = &mockito::server_url();

        let response = r#"{
            "ok": true,
            "result": [
                {
                    "update_id": 500612408,
                    "message": {
                        "message_id": 71,
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
                        "date": 1652924741,
                        "text": "vai"
                    }
                }
            ]
        }"#;

        let _m = mock("GET", "token/getUpdates")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response)
            .create();

        let result = super::get(host.to_string(), "token".to_string()).await?;

        assert_eq!(result[0].update_id, 500612408);

        Ok(())
    }
}
