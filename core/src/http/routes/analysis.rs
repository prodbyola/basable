use std::collections::HashMap;

use axum::{
    extract::{Query, State}, http::StatusCode, routing::get, Json, Router
};
use axum_macros::debug_handler;

use crate::{
    base::{
        imp::analysis::{chrono::ChronoAnalysisOpts, AnalysisResults},
        AppError, AppState,
    },
    http::middlewares::{AuthExtractor, DbExtractor},
};

#[debug_handler]
pub async fn chrono_analysis(
    Query(params): Query<HashMap<String, String>>,
    AuthExtractor(_): AuthExtractor,
    DbExtractor(db): DbExtractor,
    State(_): State<AppState>,
) -> Result<Json<AnalysisResults>, AppError> {
    let table = params.get("table");
    let column = params.get("column");
    let basis = params.get("basis");
    let range = params.get("range");

    match (table, column, basis, range) {
        (Some(table), Some(column), Some(basis), Some(range)) => {
            let basis = basis.to_owned().try_into();
            let basis = basis
                .map_err(|err: String| AppError::new(StatusCode::EXPECTATION_FAILED, err.as_str()));

            let range = range.to_owned().try_into();
            let range = range
                .map_err(|err: String| AppError::new(StatusCode::EXPECTATION_FAILED, err.as_str()));

            let opts = ChronoAnalysisOpts {
                table: table.to_owned(),
                chrono_col: column.to_owned(),
                basis: basis?,
                range: range?,
            };

            let results = db.chrono_analysis(opts)?;

            Ok(Json(results))
        }
        _ => Err(AppError::new(
            StatusCode::EXPECTATION_FAILED,
            "Missing query parameters",
        )),
    }
}

pub(super) fn analysis_routes() -> Router<AppState> {
    Router::new()
        .route("/chrono", get(chrono_analysis))
}