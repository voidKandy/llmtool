use serde::{Deserialize, Serialize};
use std::env::var;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

pub struct CacheConfig {
    username: String,
    password: String,
    namespace: String,
    database: String,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            username: var("SURREAL_USER").unwrap(),
            password: var("SURREAL_PASS").unwrap(),
            namespace: var("SURREAL_NAMESPACE").unwrap(),
            database: var("SURREAL_MEM_DATABASE").unwrap(),
        }
    }
}

impl CacheConfig {
    pub async fn connect(&self) -> surrealdb::Result<Surreal<Db>> {
        let db = Surreal::new::<Mem>(()).await?;

        let creds = surrealdb::opt::auth::Root {
            username: &self.username,
            password: &self.password,
        };
        db.signin(creds).await?;
        db.use_db(&self.database).await?;
        db.use_ns(&self.namespace).await?;
        db.health().await.unwrap();

        Ok(db)
    }
}
