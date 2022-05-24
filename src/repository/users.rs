use mongodb::Database;
use crate::models::user::User;
use mongodb::Collection;

pub struct Users {
    collection: Collection<User>,
}

impl Users {
    pub fn new(db: Database) -> Self {
        Users {
            collection: db.collection("users"),
        }
    }

    pub async fn insert(&self, user: &mut User) -> Result<(), mongodb::error::Error> {
        self.collection.insert_one(user, None).await?;
        Ok(())
    }
}
