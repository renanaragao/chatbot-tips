use crate::models::user::User;
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::options::ReplaceOptions;
use mongodb::Collection;
use mongodb::Database;

#[async_trait]
pub trait IUserRepository {
    async fn save(&self, db: &Database, user: &mut User) -> Result<(), mongodb::error::Error>;
    async fn get(
        &self,
        db: &Database,
        id: i64,
    ) -> Result<std::option::Option<User>, mongodb::error::Error>;
}

pub struct UserRepository();

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
impl IUserRepository for UserRepository {
    async fn save(&self, db: &Database, user: &mut User) -> Result<(), mongodb::error::Error> {
        get_collection(db)
            .replace_one(
                doc! {"_id": user.id},
                user,
                ReplaceOptions::builder().upsert(true).build(),
            )
            .await?;
        Ok(())
    }

    async fn get(
        &self,
        db: &Database,
        id: i64,
    ) -> Result<std::option::Option<User>, mongodb::error::Error> {
        get_collection(db).find_one(doc! {"_id": id}, None).await
    }
}

fn get_collection(db: &Database) -> Collection<User> {
    db.collection::<User>("users")
}

#[cfg(test)]
mod tests {

    use crate::models::user::User;
    use crate::repository::user::UserRepository;

    use crate::repository::user::IUserRepository;
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
                let mut db = get_db().await.unwrap();

                let mut user = create_user(id);

                UserRepository().save(&mut db, &mut user).await.unwrap();

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
                let mut db = get_db().await.unwrap();

                let mut user = create_user(id);

                insert_user(&mut user).await.unwrap();

                let mut changed_user = User {
                    id: user.id,
                    first_name: String::from("Renan Aragão"),
                    last_name: String::from("Ferreira"),
                    language_code: String::from("en"),
                    is_bot: true,
                };

                UserRepository().save(&mut db, &mut changed_user).await.unwrap();

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
                let mut db = get_db().await.unwrap();

                let mut user = create_user(id);

                insert_user(&mut user).await.unwrap();

                let find = UserRepository().get(&mut db, user.id).await.unwrap();

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
    async fn get_db() -> Result<Database, mongodb::error::Error> {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        let client = Client::with_options(client_options)?;
        Ok(client.database("chat-tip-tests"))
    }
}

#[cfg(test)]
pub struct UserRepositoryFake();

#[cfg(test)]
#[async_trait]
impl IUserRepository for UserRepositoryFake {
    async fn save(&self, db: &Database, user: &mut User) -> Result<(), mongodb::error::Error> {
        let mut repository_mock = MockUserRepository::new();

        let db_clone = db.clone();

        assert_eq!(638061488, user.id);
        assert_eq!("Renan", user.first_name);
        assert_eq!("Aragão", user.last_name);
        assert_eq!("en", user.language_code);
        assert_eq!(false, user.is_bot);
        assert_eq!("chat-tip", db.name());

        repository_mock
            .expect_save()
            .times(1)
            .return_const(Ok(()));

        repository_mock.save(&db_clone, user).await?;

        Ok(())
    }

    async fn get(
        &self,
        _db: &Database,
        _id: i64,
    ) -> Result<std::option::Option<User>, mongodb::error::Error> {
        let mut repository_mock = MockUserRepository::new();
        repository_mock.expect_save().times(1).return_const(Ok(()));

        Ok(None)
    }
}
