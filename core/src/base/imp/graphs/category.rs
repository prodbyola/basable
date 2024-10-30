use std::{collections::HashMap, fmt::Display};

use axum::http::StatusCode;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    base::query::{BasableQuery, QueryOperation},
    AppError,
};

use super::FromQueryParams;

#[derive(EnumIter)]
pub enum CategoryAnalysis {
    Simple,
    ManyToMany,
    Manual,
}

impl Display for CategoryAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cat = match self {
            CategoryAnalysis::Simple => "simple",
            CategoryAnalysis::ManyToMany => "complex",
            CategoryAnalysis::Manual => "manual",
        };

        write!(f, "{cat}")
    }
}

impl TryFrom<&String> for CategoryAnalysis {
    type Error = AppError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let cats = CategoryAnalysis::iter();

        for cat in cats {
            if &cat.to_string() == value {
                return Ok(cat);
            }
        }

        Err(AppError::HttpError(
            StatusCode::EXPECTATION_FAILED,
            "invalid category_graph_type".to_string(),
        ))
    }
}

pub struct CategoryGraphOpts {
    pub table: String,
    pub analysis: CategoryAnalysis,
    pub target_column: String,
    pub limit: Option<usize>,
}

impl FromQueryParams for CategoryGraphOpts {
    fn from_query_params(params: HashMap<String, String>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let table = params.get("table");
        let analysis = params.get("analysis");
        let target_column = params.get("target_column");
        let cat_limit = params.get("limit");

        match (table, analysis, target_column) {
            (Some(table), Some(graph_type), Some(target_column)) => {
                let table = table.to_string();
                let analysis = graph_type.try_into()?;
                let target_column = target_column.to_string();

                let limit = cat_limit.map_or(Ok(0), |lmt| {
                    lmt.parse::<usize>().map_err(|err| {
                        AppError::HttpError(StatusCode::EXPECTATION_FAILED, err.to_string())
                    })
                }).ok();

                let opts = CategoryGraphOpts {
                    table,
                    analysis,
                    target_column,
                    limit,
                };

                Ok(opts)
            }
            _ => Err(AppError::HttpError(
                StatusCode::EXPECTATION_FAILED,
                "missing required parameter".to_string(),
            )),
        }
    }
}

impl From<CategoryGraphOpts> for BasableQuery {
    fn from(value: CategoryGraphOpts) -> Self {
        let CategoryGraphOpts {
            table,
            analysis,
            target_column,
            limit,
        } = value;

        let selections = vec!["COUNT(*) as COUNT".to_string(), target_column.clone()];
        let operation = QueryOperation::SelectData(Some(selections));

        BasableQuery {
            table,
            operation,
            group_by: Some(vec![target_column]),
            row_count: limit,
            ..Default::default()
        }
    }
}
