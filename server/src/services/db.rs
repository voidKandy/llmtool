use serde::{Deserialize, Serialize};
use std::env::var;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{RecordId, Surreal};

const NAMESPACE: &str = "NAMESPACE";
const DATABASE: &str = "DATABASE";

pub struct DatabaseConfig {
    username: String,
    password: String,
    namespace: String,
    database: String,
    host: String,
    port: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Record<T> {
    pub id: RecordId,
    #[serde(flatten)]
    pub obj: T,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            username: var("SURREAL_USER").unwrap(),
            password: var("SURREAL_PASS").unwrap(),
            namespace: NAMESPACE.to_owned(),
            database: DATABASE.to_owned(),
            host: var("SURREAL_HOST").unwrap(),
            port: var("SURREAL_PORT").unwrap(),
        }
    }
}

impl DatabaseConfig {
    pub async fn connect(&self) -> surrealdb::Result<Surreal<Client>> {
        let creds = surrealdb::opt::auth::Root {
            username: &self.username,
            password: &self.password,
        };
        let db = Surreal::new::<Ws>(format!("{}:{}", &self.host, &self.port)).await?;

        db.use_ns(&self.namespace).await?;
        db.use_db(&self.database).await?;
        db.signin(creds).await?;

        db.health().await.unwrap();

        Ok(db)
    }
}

mod tests {
    use serde::Deserialize;
    use surrealdb::RecordId;

    use super::DatabaseConfig;
    use crate::handlers::users::User;
    use std::env::var;

    #[derive(Debug, Deserialize)]
    struct Record {
        id: RecordId,
    }

    // https://surrealdb.com/docs/sdk/rust/methods/create
    #[tokio::test]
    async fn insert_user() {
        dotenv::dotenv().ok();
        let cfg = DatabaseConfig {
            username: var("TEST_SURREAL_USER").unwrap(),
            password: var("TEST_SURREAL_PASS").unwrap(),
            namespace: super::NAMESPACE.to_owned(),
            database: super::DATABASE.to_owned(),
            host: var("TEST_SURREAL_HOST").unwrap(),
            port: var("TEST_SURREAL_PORT").unwrap(),
        };
        let user = User {
            id: None,
            username: "Ben".to_owned(),
        };
        let db = cfg.connect().await.unwrap();

        // delete all users
        db.delete::<Vec<User>>("user").await.unwrap();
        // creates a user with a random id
        let inserted_user: Option<User> = db.create("user").content(user).await.unwrap();
        assert!(inserted_user.is_some());

        assert!(inserted_user.unwrap().id.is_some());
        // Create a record with a specific ID
        // let record: Option<Record> = db.create(("person", "tobie")).content(user).await.unwrap();
    }
}
