use crate::URI;
use reqwest::Error;
use async_trait::async_trait;

use super::models::UriTelegram;

#[async_trait]
pub trait ITelegramService {
    async fn post(&self, uri: &UriTelegram, chat_id: i64, text: &str) -> Result<(), Error>;
}

pub struct TelegramService();

#[async_trait]
impl ITelegramService for TelegramService {
    async fn post(&self, uri: &UriTelegram, chat_id: i64, text: &str) -> Result<(), Error> {
        let request_url = URI::new(&uri.scheme)
            .host(&uri.host)
            .port(uri.port)
            .path(format!("bot{token}", token = &uri.token).as_str())
            .path("sendMessage")
            .query("chat_id", chat_id)
            .query("text", text)
            .build();
    
        let client = reqwest::Client::new();
        let _ = client.post(request_url).send().await?;
    
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use httptest::{matchers::*, responders::*, Expectation, Server};

    use crate::telegram::models::UriTelegram;

    use super::{TelegramService, ITelegramService};

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

        let uri_telegram = UriTelegram::new(
            url.host().unwrap().to_string(),
            "token".to_string(),
            url.scheme_str().unwrap().to_string(),
            url.port_u16().unwrap()
        );

        let _ = TelegramService().post(
            &uri_telegram,
            638061488,
            "vai",
        )
        .await
        .unwrap();
    }
}
