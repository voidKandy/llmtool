use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

// the output to our `create_user` handler
#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// if none, struct was created on serverside
    /// if some, its been returned from database
    pub id: Option<Thing>,
    pub username: String,
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

pub async fn get_users() -> &'static str {
    "This is users"
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: None,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
