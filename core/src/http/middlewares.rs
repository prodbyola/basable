use axum::{
    async_trait,
    extract::{rejection::PathRejection, FromRef, FromRequestParts, Path},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    RequestPartsExt,
};

use crate::base::{
    imp::{table::SharedTable, SharedDB}, user::{decode_jwt, User}, AppError, AppState
};

/// Extracts information about the current [`User`] by inspecting the Authorization
/// header. If Authorization is not provided, it checks for `B-Session-Id`, which should
/// be provided for guest users. If none of this is found, the `User` is `None`.
pub(crate) struct AuthExtractor(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AuthExtractor
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut auth_header = parts.headers.get(AUTHORIZATION);

        // If Authorization header does not exist, use session-id to retrieve guest user.
        if let None = auth_header {
            auth_header = parts.headers.get("B-Session-Id");
        }

        match auth_header {
            Some(hv) => match decode_jwt(hv) {
                Ok(user) => Ok(AuthExtractor(user)),
                Err(e) => Err(e),
            },
            None => {
                let err = AppError::new(
                    StatusCode::UNAUTHORIZED,
                    "User authentication not provided.",
                );
                return Err(err);
            }
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
        let user = user.0;

        let state = extract_app_state(parts, state).await;
        
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
        
        let bsbl = state.instance.lock().unwrap();
        let db = bsbl.get_connection(conn_id.unwrap(), &user.id);
        std::mem::drop(bsbl); // release Mutex lock

        match db {
            Some(db) => Ok(DbExtractor(db)),
            None => Err(AppError::new(
                StatusCode::PRECONDITION_FAILED,
                "You do not have access to this connection.",
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
