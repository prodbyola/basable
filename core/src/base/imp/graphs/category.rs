use std::{collections::HashMap, fmt::Display};

use axum::http::StatusCode;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::base::{query::{BasableQuery, QueryOperation}, AppError};

use super::FromQueryParams;

#[derive(EnumIter)]
pub enum CategoryGraphType {
    Simple,
    ManyToMany,
    Manual
}

impl Display for CategoryGraphType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cat = match self {
            CategoryGraphType::Simple => "simple",
            CategoryGraphType::ManyToMany => "complex",
            CategoryGraphType::Manual => "manual",
        };

        write!(f, "{cat}")
    }
}

impl TryFrom<&String> for CategoryGraphType {
    type Error = AppError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let cats = CategoryGraphType::iter();

        for cat in cats {
            if &cat.to_string() == value {
                return Ok(cat)
            }
        };

        Err(AppError::new(StatusCode::EXPECTATION_FAILED, "invalid category_graph_type"))
    }
}

pub struct CategoryGraphOpts {
    pub table: String,
    pub graph_type: CategoryGraphType,
    pub target_column: String,
    pub limit: Option<usize>
}

impl FromQueryParams for CategoryGraphOpts {
    fn from_query_params(params: HashMap<String, String>) -> Result<Self, AppError> where Self: Sized {
        let table = params.get("table");
    let graph_type = params.get("graph_type");
    let target_column = params.get("target_column");
    let cat_limit = params.get("limit");

    match (table, graph_type, target_column) {
        (Some(table), Some(graph_type), Some(target_column)) => {
            let table = table.to_string();
            let graph_type = graph_type.try_into()?;
            let target_column = target_column.to_string();

            let mut limit = None;
            if let Some(lmt) = cat_limit {
                let parse_limit = lmt.parse::<usize>();

                if let Err(err) = parse_limit {
                    return Err(AppError::new(
                        StatusCode::EXPECTATION_FAILED,
                        err.to_string().as_str(),
                    ));
                }

                // it's safe to unwrap since we checked and returned error earlier
                limit = Some(parse_limit.unwrap())
            }

            let opts = CategoryGraphOpts { 
                table, 
                graph_type, 
                target_column, 
                limit 
            };

            Ok(opts)
                
        },
        _ => Err(AppError::new(
            StatusCode::EXPECTATION_FAILED,
            "missing required parameter",
        )),
    }

    }
}

impl From<CategoryGraphOpts> for BasableQuery {
    fn from(value: CategoryGraphOpts) -> Self {
        let CategoryGraphOpts {
            table,
            graph_type,
            target_column,
            limit,
        } = value;
        
        let select_columns = vec!["COUNT(*) as COUNT".to_string(), target_column.clone()];
        let operation = QueryOperation::SelectData(Some(select_columns));

        BasableQuery {
            table,
            operation,
            group_by: Some(vec![target_column]),
            limit,
            ..Default::default()
        }
    }
}