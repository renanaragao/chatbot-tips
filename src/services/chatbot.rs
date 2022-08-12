use async_trait::async_trait;
use dilib::resolve;
use reqwest::Error;

use crate::telegram::models::{Update, UriTelegram};
use crate::ITelegramService;

#[async_trait]
pub trait IChatbotService {
    async fn new_update(&self, uri_telegram: &UriTelegram, update: Update)
        -> Result<(), Error>;
}

pub struct ChatbotService();

#[async_trait]
impl IChatbotService for ChatbotService {
    async fn new_update(
        &self,
        uri_telegram: &UriTelegram,
        update: Update,
    ) -> Result<(), Error> {
        let resolve = resolve!(trait ITelegramService).unwrap();
        let telegram_service = resolve.as_ref();

        if update.message.text.eq("/help") {
            telegram_service
            .post(uri_telegram, update.message.chat.id, "Olá, eu sou o Bot Tips. Em breve irei te ajudar a não esquecer o que você aprendeu")
            .await?
        }

        Ok(())
    }
}

#[cfg(test)]
pub struct ChatbotServiceFake();

#[cfg(test)]
#[async_trait]
impl IChatbotService for ChatbotServiceFake {
    async fn new_update(
        &self,
        uri_telegram: &UriTelegram,
        update: Update,
    ) -> Result<(), Error> {
        Ok(())
    }
}
