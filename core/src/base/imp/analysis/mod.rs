use std::fmt::{Debug, Display};

use chrono::ChronoAnalysisOpts;
use time::Date;
use trend::TrendAnalysisOpts;

use super::db::DBError;

pub(crate) mod chrono;
pub(crate) mod trend;

pub(crate) type AnalysisResults = Vec<AnalysisResult>;

pub(crate) enum AnalysisValue {
    Int(usize),
    Text(String),
    Date(Date),
    Float(f32),
    Double(f64)
}

impl Display for AnalysisValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            AnalysisValue::Int(value) => value.to_string(),
            AnalysisValue::Text(value) => value.to_string(),
            AnalysisValue::Date(value) => value.to_string(),
            AnalysisValue::Float(value) => value.to_string(),
            AnalysisValue::Double(value) => value.to_string(),
        };

        write!(f, "{}", value)

    }
}

pub(crate) struct AnalysisResult(AnalysisValue, AnalysisValue);
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

pub(crate) trait AnalyzeDB {
    fn chrono_analysis(&self, opts: ChronoAnalysisOpts) -> Result<AnalysisResults, DBError>;
    fn trend_analysis(&self, opts: TrendAnalysisOpts) -> Result<AnalysisResults, DBError>;
}

#[cfg(test)]
mod tests {
    use crate::{
        base::{
            imp::analysis::chrono::{ChronoAnalysisBasis, ChronoAnalysisRange},
            AppError,
        },
        tests::common::create_test_db,
    };

    use super::{trend::{TrendAnalysisOpts, TrendAnalysisOrder, TrendAnalysisType}, ChronoAnalysisOpts};

    #[test]
    fn test_chrono_analysis() -> Result<(), AppError> {
        let db = create_test_db()?;
        let analyze = db.chrono_analysis(ChronoAnalysisOpts {
            table: "vgchartz".to_string(),
            chrono_col: "release_date".to_string(),
            basis: ChronoAnalysisBasis::Monthly,
            range: ChronoAnalysisRange("2010-09-01".to_string(), "2010-11-30".to_string()),
        });

        assert!(analyze.is_ok());

        Ok(())
    }

    #[test]
    fn test_trend_analysis() -> Result<(), AppError> {
        let db = create_test_db()?;
        let opts = TrendAnalysisOpts {
            table: "vgchartz".to_string(),
            analysis_type: TrendAnalysisType::IntraModel,
            xcol: "title".to_string(),
            ycol: "total_sales".to_string(),
            order: TrendAnalysisOrder::ASC,
            limit: 10,
        };

        let analyze = db.trend_analysis(opts);
        assert!(analyze.is_ok());

        Ok(())
    }
}
