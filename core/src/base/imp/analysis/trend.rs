use std::fmt::Display;

use axum::http::StatusCode;

use crate::base::AppError;

pub enum TrendAnalysisType {
    IntraModel,
    CrossModel,
}
pub enum TrendAnalysisOrder {
    DESC,
    ASC,
}

impl Display for TrendAnalysisOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let order = match self {
            TrendAnalysisOrder::DESC => "DESC",
            TrendAnalysisOrder::ASC => "ASC",
        };

        write!(f, "{}", order)
    }
}

pub struct CrossOptions {
    /// The foreign table we want to connect to.
    pub foreign_table: String,

    /// Column on [`TrendAnalysisOpts::table`] that should be targeted by [`TrendAnalysisOpts::ycol`].
    pub target_col: String,
}

pub struct TrendAnalysisOpts {
    /// The primary table you want to analyze
    pub table: String,

    /// The type of trend analysis you want want to perform 
    pub analysis_type: TrendAnalysisType,

    /// The column you want to use for independent variables.
    pub xcol: String,

    /// The column you want to use for dependent variable. For [`TrendAnalysisType::CrossModel`],
    /// this should be set to the column name used as foreignKey from [`CrossOptions::foreign_table`].
    pub ycol: String,

    /// Order of the analysis
    pub order: TrendAnalysisOrder,

    /// Limit of returned analysis
    pub limit: usize,

    /// Configure this option if you're using [`TrendAnalysisType::CrossModel`].
    pub cross: Option<CrossOptions>
}

impl TrendAnalysisOpts {
    pub fn build_query(&self) -> Result<String, AppError> {
        let TrendAnalysisOpts {
            table,
            analysis_type,
            xcol,
            ycol,
            order,
            limit,
            cross,
        } = self;

        match analysis_type {
            TrendAnalysisType::IntraModel => {
                let q = format!(
                    "
                        SELECT {xcol}, {ycol} 
                        FROM {table} 
                        ORDER BY {ycol} {order} 
                        LIMIT {limit}
                    "
                );

                Ok(q)
            }
            TrendAnalysisType::CrossModel => match cross {
                Some(cross) => {
                    let CrossOptions { foreign_table, target_col } = cross;
                    let q = format!(
                        "
                            SELECT x.{xcol} AS {xcol}, COUNT(y.{ycol}) AS {ycol} 
                            FROM {table} x 
                            LEFT JOIN {foreign_table} y ON x.{target_col} = y.{ycol} 
                            GROUP BY {xcol} 
                            HAVING {ycol} > 0
                            ORDER BY {ycol} {order}
                            LIMIT {limit} 
                        "
                    );

                    Ok(q)
                }
                None => {
                    let err = AppError::new(
                        StatusCode::EXPECTATION_FAILED,
                        "You must provide cross model options.",
                    );
                    Err(err)
                }
            },
        }
    }
}
