use crate::URI;
use reqwest::Error;

pub async fn post<'a>(uri: &'a mut URI<'a>, chat_id: i64, text: &str) -> Result<(), Error> {
    let request_url = uri
        .path("sendMessage")
        .query("chat_id", chat_id)
        .query("text", text)
        .build();

    let client = reqwest::Client::new();
    let _ = client.post(request_url).send().await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use httptest::{matchers::*, responders::*, Expectation, Server};
    use uri_builder::URI;

    #[tokio::test]
    async fn should_send_message() {
        let server = Server::run();

        server.expect(
            Expectation::matching(all_of!(
                request::method("POST"),
                request::path("/bottoken/sendMessage"),
                request::query(url_decoded(contains(("chat_id", "638061488")))),
                request::query(url_decoded(contains(("text", "vai")))),
            ))
            .respond_with(status_code(200)),
        );

        let url = server.url("/bottoken/sendMessage?chat_id=638061488&text=vai");

        let _ = super::post(
            URI::new(url.scheme_str().unwrap())
                .host(url.host().unwrap())
                .port(url.port_u16().unwrap())
                .path("bottoken"),
            638061488,
            "vai",
        )
        .await
        .unwrap();
    }
}
