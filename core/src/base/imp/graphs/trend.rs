use std::{collections::HashMap, fmt::Display};

use axum::http::StatusCode;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    base::query::{
        filter::{Filter, FilterChain, FilterCombinator, FilterExpression},
        BasableQuery, QueryOperation, QueryOrder,
    },
    AppError,
};

use super::FromQueryParams;

#[derive(Clone)]
pub enum TrendGraphType {
    IntraModel,
    CrossModel,
}

impl TryFrom<&String> for TrendGraphType {
    type Error = AppError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value == "intra" {
            Ok(Self::IntraModel)
        } else if value == "cross" {
            Ok(Self::CrossModel)
        } else {
            Err(AppError::HttpError(
                StatusCode::EXPECTATION_FAILED,
                "Invalid TrendGraphType".to_string(),
            ))
        }
    }
}

#[derive(EnumIter)]
pub enum TrendGraphOrder {
    DESC,
    ASC,
}

impl Display for TrendGraphOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let order = match self {
            TrendGraphOrder::DESC => "DESC",
            TrendGraphOrder::ASC => "ASC",
        };

        write!(f, "{}", order)
    }
}

impl TryFrom<&String> for TrendGraphOrder {
    type Error = AppError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        for order in TrendGraphOrder::iter() {
            if &order.to_string() == value {
                return Ok(order);
            }
        }

        Err(AppError::HttpError(
            StatusCode::EXPECTATION_FAILED,
            "Error parsing TrendOrder".to_string(),
        ))
    }
}

pub struct CrossOptions {
    /// The foreign table we want to connect to.
    pub foreign_table: String,

    /// Column on [`TrendAnalysisOpts::table`] that should be targeted by [`TrendAnalysisOpts::ycol`].
    pub target_col: String,
}

pub struct TrendGraphOpts {
    /// The primary table you want to analyze
    pub table: String,

    /// The type of trend analysis you want want to perform
    pub graph_type: TrendGraphType,

    /// The column you want to use for independent variables.
    pub xcol: String,

    /// The column you want to use for dependent variable. For [`TrendGraphType::CrossModel`],
    /// this should be set to the column name used as foreignKey from [`CrossOptions::foreign_table`].
    pub ycol: String,

    /// Order of the analysis
    pub order: Option<TrendGraphOrder>,

    /// Limit of returned analysis
    pub limit: Option<usize>,

    /// Configure this option if you're using [`TrendAnalysisType::CrossModel`].
    pub cross: Option<CrossOptions>,
}

impl FromQueryParams for TrendGraphOpts {
    fn from_query_params(params: HashMap<String, String>) -> Result<Self, AppError> {
        let table = params.get("table");
        let graph_type = params.get("graph_type");
        let xcol = params.get("xcol");
        let ycol = params.get("ycol");
        let trend_order = params.get("order");
        let trend_limit = params.get("limit");
        let foreign_table = params.get("foreign_table");
        let target_column = params.get("target_column");

        match (table, graph_type, xcol, ycol) {
            (Some(table), Some(graph_type), Some(xcol), Some(ycol)) => {
                let graph_type = graph_type.try_into()?;

                // parse query order
                let mut order = None;
                if let Some(tod) = trend_order {
                    order = Some(tod.try_into()?)
                }

                // parse query limit
                let limit = trend_limit.map_or(Ok(0), |lmt| {
                    lmt.parse::<usize>().map_err(|err| {
                        AppError::HttpError(StatusCode::EXPECTATION_FAILED, err.to_string())
                    })
                }).ok();
                
                // parse cross analysis options
                let mut cross_err = Ok(());
                let mut cross = None;

                // Check if all parameters are provided for cross analysis
                match (foreign_table, target_column) {
                    (None, None) => cross = None,
                    (None, Some(_)) => {
                        cross_err = Err(AppError::HttpError(
                            StatusCode::EXPECTATION_FAILED,
                            "missing 'foreign_table' parameter".to_string(),
                        ))
                    }
                    (Some(_), None) => {
                        cross_err = Err(AppError::HttpError(
                            StatusCode::EXPECTATION_FAILED,
                            "missing 'target_column' parameter".to_string(),
                        ))
                    }
                    (Some(ft), Some(tc)) => {
                        cross = Some(CrossOptions {
                            foreign_table: ft.to_string(),
                            target_col: tc.to_string(),
                        })
                    }
                }

                // if insufficient parameters are supplied for cross analysis, return error
                if let Err(err) = cross_err {
                    return Err(err);
                }

                let opts = TrendGraphOpts {
                    table: String::from(table),
                    graph_type,
                    xcol: String::from(xcol),
                    ycol: String::from(ycol),
                    order,
                    limit,
                    cross,
                };

                Ok(opts)
            }
            _ => {
                let err = AppError::HttpError(
                    StatusCode::EXPECTATION_FAILED,
                    "Missing query parameters".to_string(),
                );
                Err(err)
            }
        }
    }
}

impl TryFrom<TrendGraphOpts> for BasableQuery {
    type Error = AppError;

    fn try_from(value: TrendGraphOpts) -> Result<Self, Self::Error> {
        let TrendGraphOpts {
            table,
            graph_type: analysis_type,
            xcol,
            ycol,
            order,
            limit,
            cross,
        } = value;

        match analysis_type {
            TrendGraphType::IntraModel => {
                let operation = QueryOperation::SelectData(Some(vec![xcol, ycol.clone()]));

                let order = match order {
                    Some(order) => match order {
                        TrendGraphOrder::DESC => QueryOrder::DESC(ycol),
                        TrendGraphOrder::ASC => QueryOrder::ASC(ycol),
                    },
                    None => QueryOrder::DESC(ycol),
                };

                let order_by = Some(order);

                let q = BasableQuery {
                    table,
                    operation,
                    order_by,
                    row_count: limit,
                    ..Default::default()
                };

                Ok(q)
            }

            TrendGraphType::CrossModel => match cross {
                Some(cross) => {
                    let CrossOptions {
                        foreign_table,
                        target_col,
                    } = cross;

                    let select_columns = vec![
                        format!("x.{xcol} AS {xcol}"),
                        format!("COUNT(y.{ycol}) AS {ycol}"),
                    ];

                    let operation = QueryOperation::SelectData(Some(select_columns));
                    let left_join = format!("{foreign_table} y ON x.{target_col} = y.{ycol}");

                    let mut having = FilterChain::new();
                    having.add_one(Filter{
                        combinator: FilterCombinator::BASE,
                        column: ycol.clone(),
                        expression: FilterExpression::Gt("0".to_string())
                    });

                    let order = match order {
                        Some(order) => match order {
                            TrendGraphOrder::DESC => QueryOrder::DESC(ycol),
                            TrendGraphOrder::ASC => QueryOrder::ASC(ycol),
                        },
                        None => QueryOrder::DESC(ycol),
                    };

                    let order_by = Some(order);

                    let q = BasableQuery {
                        operation,
                        having,
                        table: format!("{table} x"),
                        left_join: Some(left_join),
                        group_by: Some(vec![xcol]),
                        order_by,
                        row_count: limit,
                        ..Default::default()
                    };

                    Ok(q)
                }
                None => {
                    let err = AppError::HttpError(
                        StatusCode::EXPECTATION_FAILED,
                        "You must provide cross model options.".to_string(),
                    );

                    Err(err)
                }
            },
        }
    }
}
