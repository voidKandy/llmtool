use super::{ErrorResponse, HttpError, HttpSuccess};
use crate::services::db::Record;
use crate::{api::Notes, handlers::AppState};
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,
}

#[derive(Serialize, Deserialize)]
pub struct NoteRecord {
    pub note: Record<Note>,
}

#[derive(Serialize, Deserialize)]
pub struct NotesRecord {
    pub notes: Vec<Record<Note>>,
}

pub async fn get_notes_handler(State(state): State<AppState>) -> Result<HttpSuccess, HttpError> {
    let api = state.api.lock().await;
    match api.get_notes().await {
        Ok(notes) => Ok(HttpSuccess::JsonNotes(notes)),
        Err(e) => Err(HttpError::InternalServerError(ErrorResponse {
            error: e.to_string(),
        })),
    }
}

pub async fn get_note_handler(
    State(state): State<AppState>,
    Path(note_id): Path<String>,
) -> Result<HttpSuccess, HttpError> {
    let api = state.api.lock().await;
    match api.get_note_by_id(&note_id).await {
        Ok(note) => Ok(HttpSuccess::JsonNote(note)),
        Err(e) => Err(HttpError::InternalServerError(ErrorResponse {
            error: e.to_string(),
        })),
    }
}

pub async fn add_note_handler(
    State(state): State<AppState>,
    Json(payload): Json<Note>,
) -> Result<HttpSuccess, HttpError> {
    let api = state.api.lock().await;
    match api.add_note(payload.title, payload.content).await {
        Ok(note) => Ok(HttpSuccess::NoteCreated(note)),
        Err(e) => Err(HttpError::InternalServerError(ErrorResponse {
            error: e.to_string(),
        })),
    }
}

pub async fn edit_note_handler(
    State(state): State<AppState>,
    Path(note_id): Path<String>,
    Json(payload): Json<Note>,
) -> Result<HttpSuccess, HttpError> {
    let api = state.api.lock().await;
    match api
        .edit_note(&note_id, payload.title, payload.content)
        .await
    {
        Ok(note) => Ok(HttpSuccess::JsonNote(note)),
        Err(e) => Err(HttpError::InternalServerError(ErrorResponse {
            error: e.to_string(),
        })),
    }
}

pub async fn delete_note_handler(
    State(state): State<AppState>,
    Path(note_id): Path<String>,
) -> Result<HttpSuccess, HttpError> {
    let api = state.api.lock().await;
    match api.delete_note(&note_id).await {
        Ok(note) => Ok(HttpSuccess::JsonNote(note)),
        Err(e) => Err(HttpError::InternalServerError(ErrorResponse {
            error: e.to_string(),
        })),
    }
}
