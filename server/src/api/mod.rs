use crate::handlers::notes::{NoteRecord, NotesRecord};
use crate::services::db::{DatabaseConfig, Record};
use std::future::Future;
use surrealdb::engine::remote::ws::Client;
use surrealdb::{Result, Surreal};

mod notes;

pub struct Api {
    // cache: Redis or sum
    db: Surreal<Client>,
}

impl Api {
    pub async fn init() -> Self {
        let db = crate::services::db::connect(DatabaseConfig::default())
            .await
            .expect("problem connecting to database");
        Self { db }
    }
}

// pub trait Users {
//     fn get_users(&self) -> impl Future<Output = &'static str> + Send;
//     fn get_user_by_id(&self) -> impl Future<Output = &'static str> + Send;
//     fn add_user(&self) -> impl Future<Output = &'static str> + Send;
//     fn edit_user(&self) -> impl Future<Output = &'static str> + Send;
//     fn delete_user(&self) -> impl Future<Output = &'static str> + Send;
// }

pub trait Notes {
    fn get_notes(&self) -> impl Future<Output = Result<NotesRecord>> + Send;
    // fn get_notes_by_category(&self) -> impl Future<Output = surrealdb::Result<()>> + Send;
    fn get_note_by_id(&self, id: &str) -> impl Future<Output = Result<NoteRecord>> + Send;
    fn add_note(
        &self,
        title: String,
        content: String,
    ) -> impl Future<Output = Result<NoteRecord>> + Send;
    fn edit_note(
        &self,
        id: &str,
        title: String,
        content: String,
    ) -> impl Future<Output = Result<NoteRecord>> + Send;
    fn delete_note(&self, id: &str) -> impl Future<Output = Result<NoteRecord>> + Send;
}
