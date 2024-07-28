use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use axum_macros::debug_handler;

use crate::{
    base::{
        imp::graphs::{
            category::CategoryGraphOpts, chrono::ChronoAnalysisOpts, geo::GeoGraphOpts,
            trend::TrendGraphOpts, AnalysisResults, FromQueryParams,
        },
        AppError, AppState,
    },
    http::middlewares::{AuthExtractor, DbExtractor},
};

#[debug_handler]
pub async fn chrono_graph(
    Query(params): Query<HashMap<String, String>>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    State(_): State<AppState>,
) -> Result<Json<AnalysisResults>, AppError> {
    let opts = ChronoAnalysisOpts::from_query_params(params)?;
    let results = db.chrono_graph(opts)?;

    Ok(Json(results))
}

#[debug_handler]
pub async fn trend_graph(
    Query(params): Query<HashMap<String, String>>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    State(_): State<AppState>,
) -> Result<Json<AnalysisResults>, AppError> {
    let opts = TrendGraphOpts::from_query_params(params)?;
    let graph = db.trend_graph(opts)?;

    Ok(Json(graph))
}

#[debug_handler]
pub async fn category_graph(
    Query(params): Query<HashMap<String, String>>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    State(_): State<AppState>,
) -> Result<Json<AnalysisResults>, AppError> {
    let opts = CategoryGraphOpts::from_query_params(params)?;
    let graph = db.category_graph(opts)?;

    Ok(Json(graph))
}

#[debug_handler]
pub async fn geo_graph(
    Query(params): Query<HashMap<String, String>>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    State(_): State<AppState>,
) -> Result<Json<AnalysisResults>, AppError> {
    let opts = GeoGraphOpts::from_query_params(params)?;
    let graph = db.geo_graph(opts)?;

    Ok(Json(graph))
}

/// A collection of routes for Graph construction
pub(super) fn graphs_routes() -> Router<AppState> {
    Router::new()
        .route("/chrono", get(chrono_graph))
        .route("/trend", get(trend_graph))
        .route("/category", get(category_graph))
        .route("/geo", get(geo_graph))
}
