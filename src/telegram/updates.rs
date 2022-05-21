use reqwest::Error;

use super::models::{Response, Update};

pub async fn get(host: &str, token: &str) -> Result<Vec<Update>, Error> {
    let request_url = format!(
        "{host}/bot{token}/getUpdates",
        token = token,
        host = host
    );

    let response = reqwest::get(request_url).await?;

    let result = response.json::<Response>().await?;

    Ok(result.result)
}

#[cfg(test)]
mod tests {

    use httptest::{Server, Expectation, matchers::*, responders::*};

    #[tokio::test]
    async fn get_updates() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/bottoken/getUpdates"))
            .respond_with(json_encoded(serde_json::json!({
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
            })))
        );

        let url = server.url("/bottoken/getUpdates");

        let host = format!("{scheme}://{host}:{port}", scheme = url.scheme().unwrap(), host = url.host().unwrap(), port = url.port().unwrap());

        let result = super::get(&host, "token").await.unwrap();

        assert_eq!(result[0].update_id, 500612408);
    }
}
