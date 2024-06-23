use axum::{
    async_trait,
    extract::{rejection::PathRejection, FromRef, FromRequestParts, Path},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    RequestPartsExt,
};

use crate::base::{table::SharedTable, user::decode_jwt, AppError, SharedDB};
use crate::http::app::AppState;

/// Extracts information about the current [`User`] by inspecting the Authorization
/// header. If Authorization is not provided, it checks for `B-Session-Id`, which should
/// be provided for guest users. If none of this is found, the `User` is `None`.
pub(crate) struct AuthExtractor(pub Option<String>);

#[async_trait]
impl<S> FromRequestParts<S> for AuthExtractor
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let mut is_guest = false;

        let state = extract_app_state(parts, state).await;

        let mut bsbl = state.instance.lock().unwrap();
        let mut auth_value = parts.headers.get(AUTHORIZATION);

        // If Authorization header does not exist, use session-id to retrieve guest user.
        if let None = auth_value {
            is_guest = true;
            auth_value = parts.headers.get("B-Session-Id");
        }

        if let Some(hv) = auth_value {
            let mut user_id = None;

            if is_guest {
                match decode_jwt(hv) {
                    Ok(id) => user_id = Some(id),
                    Err(e) => {
                        if let Some(id) = user_id {
                            bsbl.log_user_out(&id);
                        }

                        return Err(e);
                    }
                };
            } else {
                // validate user from remote server
                let err = AppError::new(StatusCode::NOT_IMPLEMENTED, "Authorization for registered users not implemented. Please use 'B-Session-Id' header.");
                return Err(err);
            }

            return Ok(AuthExtractor(user_id));
        } else {
            let err = AppError::new(
                StatusCode::UNAUTHORIZED,
                "User authentication not provided.",
            );
            return Err(err);
        }
    }
}

pub struct DbExtractor(pub SharedDB);

#[async_trait]
impl<S> FromRequestParts<S> for DbExtractor
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = AuthExtractor::from_request_parts(parts, state).await?;
        match user.0 {
            Some(user_id) => {
                let state = extract_app_state(parts, state).await;
                let bsbl = state.instance.lock().unwrap();

                let conn_id = match parts.headers.get("Connection-Id") {
                    Some(h) => Some(h.to_str().unwrap()),
                    None => None,
                };

                if let None = conn_id {
                    return Err(AppError::new(
                        StatusCode::PRECONDITION_REQUIRED,
                        "Connection Id not provided",
                    ));
                }

                let db = bsbl.get_connection(conn_id.unwrap(), &user_id);
                match db {
                    Some(db) => Ok(DbExtractor(db)),
                    None => Err(AppError::new(
                        StatusCode::PRECONDITION_FAILED,
                        "You do not have access to this connection.",
                    )),
                }
            }
            None => Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "User authentication not provided.",
            )),
        }
    }
}

pub struct TableExtractor(pub SharedTable);

#[async_trait]
impl<S> FromRequestParts<S> for TableExtractor
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path: Result<Path<String>, PathRejection> =
            Path::from_request_parts(parts, state).await;

        match path {
            Ok(path) => {
                let Path(tbl_name) = path;
                let ext = DbExtractor::from_request_parts(parts, state).await?;
                let db = ext.0;

                match db.get_table(&tbl_name) {
                    Some(tbl) => Ok(TableExtractor(tbl.clone())),
                    None => Err(AppError::new(
                        StatusCode::NOT_FOUND,
                        "Can't find a table with the given name",
                    )),
                }
            }
            Err(err) => Err(AppError::new(
                StatusCode::PRECONDITION_FAILED,
                &err.to_string(),
            )),
        }
    }
}

/// Extract app state and get basable instance
/// https://docs.rs/axum/0.6.4/axum/extract/struct.State.html#for-library-authors
async fn extract_app_state<S>(parts: &mut Parts, state: &S) -> AppState
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    parts
        .extract_with_state::<AppState, _>(state)
        .await
        .unwrap()
}
