use crate::api::Api;
use crate::MainResult;
use auth::CurrentUser;
use axum::http::{Error, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{
    http::{header, HeaderValue, Method},
    routing::{delete, patch},
    routing::{get, post},
    Router,
};
use axum::{middleware, Json};
use notes::{Note, NoteRecord, NotesRecord};
use serde::{Deserialize, Serialize};
use std::env::var;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod notes;
pub mod users;

pub struct App {
    listener: TcpListener,
    router: axum::Router,
}

#[derive(Clone)]
pub struct AppState {
    api: Arc<Mutex<Api>>,
}

pub enum HttpSuccess {
    Ok,
    Created,
    NoteCreated(NoteRecord),
    JsonNote(NoteRecord),
    JsonNotes(NotesRecord),
    UserData(CurrentUser),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Serialize, Deserialize)]
pub enum HttpError {
    BadRequest(ErrorResponse),
    Unauthorised,
    InternalServerError(ErrorResponse),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
            Self::Unauthorised => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: format!("unauthorized, access denied"),
                }),
            )
                .into_response(),
            Self::InternalServerError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response()
            }
        }
    }
}

impl IntoResponse for HttpSuccess {
    fn into_response(self) -> Response {
        match self {
            Self::Ok => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::NoteCreated(data) => (StatusCode::CREATED, Json(data)).into_response(),
            Self::JsonNote(data) => (StatusCode::OK, Json(data)).into_response(),
            Self::JsonNotes(data) => (StatusCode::OK, Json(data)).into_response(),
            Self::UserData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

impl App {
    pub async fn init() -> Self {
        dotenv::dotenv().ok();
        let host = var("SERVER_HOST").unwrap();
        let port = var("SERVER_PORT").unwrap();

        let api = Api::init().await;
        let app_state = AppState {
            api: Arc::new(Mutex::new(api)),
        };

        let complete_url: String = format!("{host}:{port}/");

        let origins = [HeaderValue::from_str(&complete_url).unwrap()];
        let cors = CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
            .allow_credentials(true);

        let router = Router::new()
            .layer(cors)
            .route("/", get(health_check))
            .route("/login", post(auth::login))
            .route("/logout", delete(auth::logout))
            .route("/notes", get(notes::get_notes_handler))
            .route("/notes", post(notes::add_note_handler))
            .route("/notes/{note_id}", get(notes::get_note_handler))
            .route("/notes/{note_id}", patch(notes::edit_note_handler))
            .route("/notes/{note_id}", delete(notes::delete_note_handler))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(middleware::from_fn(auth::auth_middleware)),
            )
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
            .await
            .expect("could not spin up TCP listener");

        tracing::warn!("LISTENING ON {}", complete_url);

        Self { router, listener }
    }

    pub async fn run(self) -> MainResult<()> {
        axum::serve(self.listener, self.router).await.unwrap();
        Ok(())
    }
}

async fn health_check() -> &'static str {
    "Dedidated serva is running!"
}

// async fn handler(
//     // extract the current user, set by the middleware
//     Extension(current_user): Extension<CurrentUser>,
// ) {
//     // ...
// }
