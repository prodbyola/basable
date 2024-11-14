use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use axum::http::StatusCode;
use category::CategoryGraphOpts;
use chrono::ChronoAnalysisOpts;
use geo::GeoGraphOpts;
use mysql::Value as MysqlValue;
use serde::{ser::SerializeTuple, Serialize};
use time::Date;
use trend::TrendGraphOpts;

use crate::AppError;

pub mod category;
pub mod chrono;
pub mod geo;
pub mod trend;

pub type AnalysisResults = Vec<AnalysisResult>;

#[derive(Default)]
pub enum AnalysisValue {
    #[default]
    NULL,
    UInt(usize),
    Int(isize),
    Text(String),
    Date(Date),
    Float(f32),
    Double(f64),
}

impl Serialize for AnalysisValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_tuple(5)?;

        match self {
            AnalysisValue::NULL => s.serialize_element(&Option::<u8>::None)?,
            AnalysisValue::UInt(uint) => s.serialize_element(uint)?,
            AnalysisValue::Int(int) => s.serialize_element(int)?,
            AnalysisValue::Text(text) => s.serialize_element(text)?,
            AnalysisValue::Date(date) => s.serialize_element(&date.to_string())?,
            AnalysisValue::Float(float) => s.serialize_element(float)?,
            AnalysisValue::Double(double) => s.serialize_element(double)?,
        }

        s.end()
    }
}

impl Display for AnalysisValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            AnalysisValue::NULL => "null".to_string(),
            AnalysisValue::UInt(value) => value.to_string(),
            AnalysisValue::Int(value) => value.to_string(),
            AnalysisValue::Text(value) => value.to_string(),
            AnalysisValue::Date(value) => value.to_string(),
            AnalysisValue::Float(value) => value.to_string(),
            AnalysisValue::Double(value) => value.to_string(),
        };

        write!(f, "{}", value)
    }
}

impl TryFrom<MysqlValue> for AnalysisValue {
    type Error = AppError;

    fn try_from(value: MysqlValue) -> Result<Self, Self::Error> {
        let s = match value {
            MysqlValue::NULL => AnalysisValue::NULL,
            MysqlValue::Bytes(v) => {
                let v = String::from_utf8(v).map_err(|err| {
                    AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                })?;
                AnalysisValue::Text(v)
            }
            MysqlValue::UInt(v) => {
                let v = usize::try_from(v).map_err(|err| {
                    AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                })?;
                AnalysisValue::UInt(v)
            }
            MysqlValue::Int(v) => {
                let v = isize::try_from(v).map_err(|err| {
                    AppError::HttpError(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                })?;
                AnalysisValue::Int(v)
            }
            MysqlValue::Float(v) => AnalysisValue::Float(v),
            MysqlValue::Double(v) => AnalysisValue::Double(v),
            _ => AnalysisValue::NULL,
        };

        Ok(s)
    }
}

#[derive(Serialize)]
pub struct AnalysisResult(AnalysisValue, AnalysisValue);
impl AnalysisResult {
    pub fn new(x: AnalysisValue, y: AnalysisValue) -> Self {
        AnalysisResult(x, y)
    }
}

impl Debug for AnalysisResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x: {}, y: {}}}", self.0, self.1)
    }
}

pub trait VisualizeDB {
    fn chrono_graph(&self, opts: ChronoAnalysisOpts) -> Result<AnalysisResults, AppError>;
    fn trend_graph(&self, opts: TrendGraphOpts) -> Result<AnalysisResults, AppError>;
    fn category_graph(&self, opts: CategoryGraphOpts) -> Result<AnalysisResults, AppError>;
    fn geo_graph(&self, opts: GeoGraphOpts) -> Result<AnalysisResults, AppError>;
}

pub trait FromQueryParams {
    fn from_query_params(params: HashMap<String, String>) -> Result<Self, AppError>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use crate::{
        base::imp::graphs::{
            category::{CategoryAnalysis, CategoryGraphOpts},
            chrono::{ChronoAnalysisBasis, ChronoAnalysisRange},
            trend::CrossOptions,
        },
        tests::common::create_test_db,
        AppError,
    };

    use super::{
        trend::{TrendGraphOpts, TrendGraphOrder, TrendGraphType},
        ChronoAnalysisOpts,
    };

    #[test]
    fn test_chrono_graph() -> Result<(), AppError> {
        let db = create_test_db()?;
        let graph = db.chrono_graph(ChronoAnalysisOpts {
            table: "vgchartz".to_string(),
            chrono_col: "release_date".to_string(),
            basis: ChronoAnalysisBasis::Monthly,
            range: ChronoAnalysisRange("2010-09-01".to_string(), "2010-11-30".to_string()),
        });

        assert!(graph.is_ok());

        Ok(())
    }

    #[test]
    fn test_trend_graph() -> Result<(), AppError> {
        let db = create_test_db()?;
        let opts = TrendGraphOpts {
            table: "patients".to_string(),
            graph_type: TrendGraphType::CrossModel,
            xcol: "FIRST".to_string(),
            ycol: "PATIENT".to_string(),
            order: Some(TrendGraphOrder::ASC),
            limit: Some(50),
            cross: Some(CrossOptions {
                foreign_table: "encounters".to_string(),
                target_col: "Id".to_string(),
            }),
        };

        let graph = db.trend_graph(opts);
        assert!(graph.is_ok());

        Ok(())
    }

    #[test]
    fn test_category_graph() -> Result<(), AppError> {
        let db = create_test_db()?;

        let opts = CategoryGraphOpts {
            table: "vgchartz".to_string(),
            analysis: CategoryAnalysis::Simple,
            target_column: "publisher".to_string(),
            limit: Some(20),
        };

        let graph = db.category_graph(opts);

        assert!(graph.is_ok());

        Ok(())
    }
}
