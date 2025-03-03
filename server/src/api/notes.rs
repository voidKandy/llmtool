use super::Notes;
use crate::{
    api::Api,
    handlers::notes::{Note, NoteRecord, NotesRecord},
};
use fastembed::TextEmbedding;
use surrealdb::{Error, Result, Uuid};

impl Notes for Api {
    async fn get_notes(&self) -> Result<NotesRecord> {
        let notes = self.db.select("notes").await?;
        if notes.len() < 1 {
            let err = Error::Db(surrealdb::error::Db::Thrown(format!(
                "there are no notes in the database",
            )));
            return Err(err);
        }
        dbg!(&notes);
        Ok(NotesRecord { notes })
    }

    // async fn get_notes_by_category(&self) -> surrealdb::Result<()> {
    //     Ok(())
    // }

    async fn get_note_by_id(&self, id: &str) -> Result<NoteRecord> {
        if let Some(note) = self.db.select(("notes", id)).await? {
            // println!("{}", note.id);
            return Ok(NoteRecord { note });
        }
        let err = Error::Db(surrealdb::error::Db::Thrown(format!(
            "note with id {} not found",
            id
        )));
        Err(err)
    }

    async fn add_note(&self, title: String, content: String) -> Result<NoteRecord> {
        // // With custom InitOptions
        // let model = TextEmbedding::try_new(
        //     InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
        // )?;
        let model = TextEmbedding::try_new(Default::default());
        match model {
            Ok(model) => {
                let documents = vec![format!("title: {}", title), format!("passage: {}", content)];
                let embedding = model.embed(documents, None);
                match embedding {
                    Ok(embedding_vec) => {
                        if let Some(note) = self
                            .db
                            // Create a record with a random ID
                            .create(("notes", Uuid::new_v4()))
                            .content(Note {
                                title,
                                content,
                                embedding: Some(embedding_vec[0].clone()),
                            })
                            .await?
                        {
                            dbg!(&note);
                            return Ok(NoteRecord { note });
                        }
                        let err = Error::Db(surrealdb::error::Db::Thrown(format!(
                            "note could not be added"
                        )));
                        return Err(err);
                    }
                    Err(err) => {
                        let err = Error::Db(surrealdb::error::Db::Thrown(format!(
                            "embedding content failed"
                        )));
                        return Err(err);
                    }
                };
            }
            Err(err) => {
                let err = Error::Db(surrealdb::error::Db::Thrown(format!(
                    "creating text embedding model failed"
                )));
                return Err(err);
            }
        }
    }

    async fn edit_note(&self, id: &str, title: String, content: String) -> Result<NoteRecord> {
        let model = TextEmbedding::try_new(Default::default());
        match model {
            Ok(model) => {
                let documents = vec![format!("title: {}", title), format!("passage: {}", content)];
                let embedding = model.embed(documents, None);
                match embedding {
                    Ok(embedding_vec) => {
                        if let Some(note) = self
                            .db
                            .update(("notes", id))
                            .merge(Note {
                                title,
                                content,
                                embedding: Some(embedding_vec[0].clone()),
                            })
                            .await?
                        {
                            dbg!(&note);
                            return Ok(NoteRecord { note });
                        }
                        let err = Error::Db(surrealdb::error::Db::Thrown(format!(
                            "note with id {} could not be edited",
                            id
                        )));
                        return Err(err);
                    }
                    Err(err) => {
                        let err = Error::Db(surrealdb::error::Db::Thrown(format!(
                            "embedding content failed"
                        )));
                        return Err(err);
                    }
                };
            }
            Err(err) => {
                let err = Error::Db(surrealdb::error::Db::Thrown(format!(
                    "creating text embedding model failed"
                )));
                return Err(err);
            }
        }
    }

    async fn delete_note(&self, id: &str) -> Result<NoteRecord> {
        if let Some(note) = self.db.delete(("notes", id)).await? {
            dbg!(&note);
            return Ok(NoteRecord { note });
        }
        let err = Error::Db(surrealdb::error::Db::Thrown(format!(
            "note with id {} could not be deleted",
            id
        )));
        Err(err)
    }
}
