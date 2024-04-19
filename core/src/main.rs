use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, MatchedPath, Request},
    http::{
        header::{ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, AUTHORIZATION, CONTENT_TYPE},
        request::Parts,
        HeaderValue, StatusCode,
    },
    routing::post,
    RequestPartsExt, Router,
};
use base::auth::{decode_jwt, User};
use base::foundation::Basable;
use http::connect;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::http::create_guest_user;

mod base;
mod http;
mod imp;

/// Extracts information about the current `User` by inspecting the Authorization
/// header. If Authorization is not provided, it checks for `B-Session-Id`, which should
/// be provided for guest users. If none of this is found, the `User` is `None`.
pub(crate) struct AuthExtractor(Option<User>);

#[async_trait]
impl<S> FromRequestParts<S> for AuthExtractor
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let mut extractor = AuthExtractor(None);
        let mut is_guest = false;

        // Get app state and basable instance
        // https://docs.rs/axum/0.6.4/axum/extract/struct.State.html#for-library-authors
        let state = parts
            .extract_with_state::<AppState, _>(state)
            .await
            .unwrap();

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

                        return Err((e.0, "User not authenticated"))
                    }
                };
            } else {
                // validate user from remote server
            }

            if let Some(user_id) = user_id {
                if let Some(user) = bsbl.users.get(&user_id)  {
                    extractor = AuthExtractor(Some(user.clone()));
                }
            }
        } else {
            return Err((StatusCode::UNAUTHORIZED, "User not authenticated"))
        }

        Ok(extractor)
    }
}

#[derive(Clone)]
pub(crate) struct AppState {
    instance: Arc<Mutex<Basable>>,
}

#[async_trait]
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "basable=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let instance = Arc::new(Mutex::new(Basable::default()));
    let state = AppState { instance };

    // We add CORS middleware to enable connection from Vue Development server
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers([ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, CONTENT_TYPE]);

    let app = Router::new()
        .route("/connect", post(connect))
        .route("/create-guest", post(create_guest_user))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|req: &Request| {
                            let method = req.method();
                            let uri = req.uri();

                            let matched_path = req
                                .extensions()
                                .get::<MatchedPath>()
                                .map(|matched_path| matched_path.as_str());

                            tracing::debug_span!("request", %method, %uri, matched_path)
                        })
                        .on_failure(()),
                )
                .layer(cors),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
