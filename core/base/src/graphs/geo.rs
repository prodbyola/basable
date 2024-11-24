use std::{collections::HashMap, fmt::Display};

use axum::http::StatusCode;
use common::{error::AppError, query::{BasableQuery, QueryCommand}};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::FromQueryParams;

#[derive(EnumIter)]
pub enum GeoGraphScope {
    Global,
    Continental,
    National,
    Regional,
}

impl Display for GeoGraphScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scope = match self {
            GeoGraphScope::Global => "global",
            GeoGraphScope::Continental => "continental",
            GeoGraphScope::National => "national",
            GeoGraphScope::Regional => "regional",
        };

        write!(f, "{scope}")
    }
}

impl TryFrom<&String> for GeoGraphScope {
    type Error = AppError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        for scope in GeoGraphScope::iter() {
            if &scope.to_string() == value {
                return Ok(scope);
            }
        }

        let iter: Vec<String> = GeoGraphScope::iter().map(|s| s.to_string()).collect();
        let scopes = iter.join(", ");
        let err = AppError::HttpError(
            StatusCode::NOT_ACCEPTABLE,
            format!("Not a valid geo scope. Acceptable options are: {scopes}."),
        );
        Err(err)
    }
}

pub struct GeoGraphOpts {
    pub table: String,
    pub scope: GeoGraphScope,
    pub target_column: String,
}

impl From<GeoGraphOpts> for BasableQuery {
    fn from(value: GeoGraphOpts) -> Self {
        let GeoGraphOpts {
            table,
            scope,
            target_column,
        } = value;

        let selections = vec!["COUNT(*) as COUNT".to_string(), target_column];
        let operation = QueryCommand::SelectData(Some(selections));

        BasableQuery {
            table,
            command: operation,
            ..Default::default()
        }
    }
}

impl FromQueryParams for GeoGraphOpts {
    fn from_query_params(params: HashMap<String, String>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let table = params.get("table");
        let scope = params.get("scope");
        let target_column = params.get("target_column");

        match (table, scope, target_column) {
            (Some(table), Some(scope), Some(target_column)) => {
                let table = table.clone();
                let scope = scope.try_into()?;
                let target_column = target_column.clone();

                let opts = GeoGraphOpts {
                    table,
                    scope,
                    target_column,
                };

                Ok(opts)
            }
            _ => {
                let err = AppError::HttpError(
                    StatusCode::EXPECTATION_FAILED,
                    "missing required parameter".to_string(),
                );
                Err(err)
            }
        }
    }
}
