use reqwest::Error;
use uri_builder::URI;

use super::models::{Response, Update};

pub async fn _get<'a>(uri: &'a mut URI<'a>) -> Result<Vec<Update>, Error> {
    let request_url = uri.path("getUpdates").build();

    let response = reqwest::get(request_url).await?;

    let result = response.json::<Response>().await?;

    Ok(result.result)
}

#[cfg(test)]
mod tests {

    use httptest::{matchers::*, responders::*, Expectation, Server};
    use uri_builder::URI;

    #[tokio::test]
    async fn should_get_updates() {
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
                }))),
        );

        let url = server.url("/bottoken/getUpdates");

        let result = super::_get(
            URI::new(url.scheme_str().unwrap())
                .host(url.host().unwrap())
                .port(url.port_u16().unwrap())
                .path("bottoken"),
        )
        .await
        .unwrap();

        assert_eq!(result[0].update_id, 500612408);
    }
}
