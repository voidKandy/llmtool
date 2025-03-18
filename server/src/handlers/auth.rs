// https://developers.google.com/identity/protocols/oauth2/native-app
use super::{HttpError, HttpSuccess};
use crate::handlers::ErrorResponse;
use axum::{extract::Request, middleware::Next, response::Response};
use axum_extra::{
    extract::cookie::{Cookie, CookieJar},
    headers::authorization::{Authorization, Bearer},
    TypedHeader,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CurrentUser {
    id: String,
    username: String,
}

pub async fn auth_middleware(
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<Response, HttpError> {
    let open_routes = ["/login", "/"];
    for i in 0..open_routes.len() {
        if req.uri() == open_routes[i] {
            return Ok(next.run(req).await);
        }
    }
    if let Some(session_id) = jar
        .get("session_id")
        .map(|cookie| cookie.value().to_owned())
    {
        if let Some(user) = current_user(&session_id).await {
            // insert the current user into a request extension so the handler can
            // extract it
            req.extensions_mut().insert(user);
            return Ok(next.run(req).await);
        }
        return Err(HttpError::Unauthorised);
    }
    if req.uri() == "/logout" {
        return Err(HttpError::BadRequest(ErrorResponse {
            error: format!("already logged out"),
        }));
    }
    Err(HttpError::Unauthorised)
}

pub async fn login(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    jar: CookieJar,
) -> Result<(CookieJar, HttpSuccess), HttpError> {
    if let Some(session_id) = authorize_current_user(auth.token()).await {
        if let Some(user) = current_user(&session_id).await {
            return Ok((
                jar.add(Cookie::new("session_id", session_id)),
                HttpSuccess::UserData(user),
            ));
        }
        return Err(HttpError::Unauthorised);
    }
    Err(HttpError::Unauthorised)
}

pub async fn logout(jar: CookieJar) -> Result<(CookieJar, HttpSuccess), HttpError> {
    // remove session from cache/db and remove cookie, use refresh token
    if let Some(session_id) = jar
        .get("session_id")
        .map(|cookie| cookie.value().to_owned())
    {
        return Ok((jar.remove(Cookie::from("session_id")), HttpSuccess::Ok));
    }
    Err(HttpError::BadRequest(ErrorResponse {
        error: format!("already logged out"),
    }))
}

// helper functions

async fn authorize_current_user(auth_token: &str) -> Option<String> {
    // create session return session id
    Some(format!("o12y3g1g31y2ut3f"))
}

async fn current_user(session_id: &str) -> Option<CurrentUser> {
    // get current user from db/cache with cookie: session_id
    Some(CurrentUser {
        id: format!("12k3h12l3h"),
        username: format!("benleem"),
    })
}
