use std::{collections::HashMap, fmt::Display};

use axum::http::StatusCode;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    base::{
        query::{
            filter::{Filter, FilterChain, FilterCondition, FilterOperator},
            BasableQuery, QueryOperation, QueryOrder,
        },
        AppError,
    },
    globals::{BASABLE_CHRONO_XCOL, BASABLE_CHRONO_YCOL},
};

use super::FromQueryParams;

#[derive(Clone, EnumIter)]
pub(crate) enum ChronoAnalysisBasis {
    Daily,
    Monthly,
    Yearly,
}

impl From<ChronoAnalysisBasis> for String {
    fn from(value: ChronoAnalysisBasis) -> Self {
        let basis = match value {
            ChronoAnalysisBasis::Daily => "Date",
            ChronoAnalysisBasis::Monthly => "Month",
            ChronoAnalysisBasis::Yearly => "Year",
        };

        basis.to_string()
    }
}

impl TryFrom<String> for ChronoAnalysisBasis {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for b in ChronoAnalysisBasis::iter() {
            let test = String::from(b.clone());
            if value == test {
                return Ok(b);
            }
        }

        Err("error parsing analysis basis".to_string())
    }
}

impl Display for ChronoAnalysisBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let basis: String = self.clone().into();
        write!(f, "{}", basis)
    }
}

pub(crate) struct ChronoAnalysisRange(pub String, pub String);
impl ChronoAnalysisRange {
    pub fn start(&self) -> &str {
        &self.0
    }

    pub fn end(&self) -> &str {
        &self.1
    }
}

impl TryFrom<String> for ChronoAnalysisRange {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let spl: Vec<&str> = value.split("range").collect();
        if spl.len() == 2 {
            let start = spl.get(0).unwrap();
            let end = spl.get(1).unwrap();

            let range = ChronoAnalysisRange(start.trim().to_string(), end.trim().to_string());
            return Ok(range);
        }

        Err("error parsing analysis range".to_string())
    }
}

pub(crate) struct ChronoAnalysisOpts {
    pub table: String,
    pub chrono_col: String,
    pub basis: ChronoAnalysisBasis,
    pub range: ChronoAnalysisRange,
}

impl FromQueryParams for ChronoAnalysisOpts {
    fn from_query_params(params: HashMap<String, String>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
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

                Ok(opts)
    
                // let results = db.chrono_graph(opts)?;
    
                // Ok(Json(results))
            }
            _ => Err(AppError::new(
                StatusCode::EXPECTATION_FAILED,
                "Missing query parameters",
            )),
        }
    
    }
}

impl From<ChronoAnalysisOpts> for BasableQuery {
    fn from(value: ChronoAnalysisOpts) -> Self {
        let ChronoAnalysisOpts {
            table,
            chrono_col,
            basis,
            range,
        } = value;

        // create query operation type
        let selections = Some(vec![
            format!("{basis}({chrono_col}) AS {BASABLE_CHRONO_XCOL}"),
            format!("COUNT(*) AS {BASABLE_CHRONO_YCOL}"),
        ]);

        let operation = QueryOperation::SelectData(selections);

        // create query filters
        let filter = Filter::BASE(FilterCondition {
            column: chrono_col.clone(),
            operator: FilterOperator::Btw(range.start().to_string(), range.end().to_string()),
        });

        let mut filters = FilterChain::new();
        filters.add_one(filter);

        // creating grouping
        let group_columns = vec![format!("{basis}({chrono_col})")];
        let group_by = Some(group_columns);

        let order_by = Some(QueryOrder::ASC(BASABLE_CHRONO_XCOL.to_string()));

        BasableQuery {
            table,
            filters,
            operation,
            group_by,
            order_by,
            ..Default::default()
        }
    }
}
