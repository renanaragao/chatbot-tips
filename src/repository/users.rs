use crate::models::user::User;
use mongodb::bson::doc;
use mongodb::options::ReplaceOptions;
use mongodb::Collection;
use mongodb::Database;

pub struct Users {
    collection: Collection<User>,
}

impl Users {
    pub fn new(db: Database) -> Self {
        Users {
            collection: db.collection("users"),
        }
    }

    pub async fn save(&self, user: &mut User) -> Result<(), mongodb::error::Error> {
        self.collection
            .replace_one(
                doc! {"_id": user.id},
                user,
                ReplaceOptions::builder().upsert(true).build(),
            )
            .await?;
        Ok(())
    }

    pub async fn get(&self, id: i64) -> Result<std::option::Option<User>, mongodb::error::Error> {
        self.collection.find_one(doc! {"_id": id}, None).await
    }
}

#[cfg(test)]
mod tests {

    use crate::models::user::User;
    use crate::repository::users::Users;

    use fake::Fake;
    use fake::Faker;
    use mongodb::bson::doc;
    use mongodb::options::ClientOptions;
    use mongodb::Client;
    use mongodb::Database;
    use try_catch::catch;

    #[tokio::test]
    async fn should_insert_user() {
        let id = Faker.fake::<i64>();

        catch! {
            try {
                let db = get_db().await.unwrap();

                let mut user = create_user(id);

                Users::new(db).save(&mut user).await.unwrap();

                let find = find_user(user.id).await.unwrap();

                delete_user(id).await.unwrap();

                match find {
                    Some(find) => {
                        assert_eq!(find.id, user.id);
                        assert_eq!(find.first_name, user.first_name);
                        assert_eq!(find.last_name, user.last_name);
                        assert_eq!(find.language_code, user.language_code);
                        assert_eq!(find.is_bot, user.is_bot);
                    },
                    None => panic!("User not found"),
                }
            }
            catch _err {
                delete_user(id).await.unwrap();
            }
        }
    }

    #[tokio::test]
    async fn should_update_user() {
        let id = Faker.fake::<i64>();

        catch! {
            try {
                let db = get_db().await.unwrap();

                let mut user = create_user(id);

                insert_user(&mut user).await.unwrap();

                let mut changed_user = User {
                    id: user.id,
                    first_name: String::from("Renan Aragão"),
                    last_name: String::from("Ferreira"),
                    language_code: String::from("en"),
                    is_bot: true,
                };

                Users::new(db).save(&mut changed_user).await.unwrap();

                let find = find_user(user.id).await.unwrap();

                delete_user(id).await.unwrap();

                match find {
                    Some(find) => {
                        assert_eq!(find, changed_user);
                    },
                    None => panic!("User not found"),
                }
            }
            catch _err {
                delete_user(id).await.unwrap();
            }
        }
    }

    #[tokio::test]
    async fn should_get_user() {
        let id = Faker.fake::<i64>();

        catch! {
            try {
                let db = get_db().await.unwrap();

                let mut user = create_user(id);

                insert_user(&mut user).await.unwrap();

                let find = Users::new(db).get(user.id).await.unwrap();

                delete_user(id).await.unwrap();

                match find {
                    Some(find) => {
                        assert_eq!(find.id, user.id);
                        assert_eq!(find.first_name, user.first_name);
                        assert_eq!(find.last_name, user.last_name);
                        assert_eq!(find.language_code, user.language_code);
                        assert_eq!(find.is_bot, user.is_bot);
                    },
                    None => panic!("User not found"),
                }
            }
            catch _err {
                delete_user(id).await.unwrap();
            }
        }
    }

    fn create_user(id: i64) -> User {
        User {
            id: id,
            first_name: String::from("Renan"),
            last_name: String::from("Aragão"),
            language_code: String::from("en"),
            is_bot: false,
        }
    }

    async fn get_db() -> Result<Database, mongodb::error::Error> {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        let client = Client::with_options(client_options)?;
        Ok(client.database("chat-tip-tests"))
    }

    async fn find_user(id: i64) -> Result<Option<User>, mongodb::error::Error> {
        let db = get_db().await?;
        let user = db
            .collection("users")
            .find_one(doc! {"_id": id}, None)
            .await?;
        Ok(user)
    }

    async fn delete_user(id: i64) -> Result<(), mongodb::error::Error> {
        let db = get_db().await?;
        db.collection::<User>("users")
            .delete_one(doc! {"_id": id}, None)
            .await?;
        Ok(())
    }

    async fn insert_user(user: &mut User) -> Result<(), mongodb::error::Error> {
        let db = get_db().await?;
        db.collection::<User>("users")
            .insert_one(user, None)
            .await?;
        Ok(())
    }
}
