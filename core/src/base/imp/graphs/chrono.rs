use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::base::query::{
    filter::{Filter, FilterChain, FilterCondition, FilterOperator},
    BasableQuery, QueryOperation, QueryOrder,
};

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
        let spl: Vec<&str> = value.split("-").collect();
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

impl Into<BasableQuery> for ChronoAnalysisOpts {
    fn into(self) -> BasableQuery {
        let ChronoAnalysisOpts {
            table,
            chrono_col,
            basis,
            range,
        } = self;

        let xcol = "BASABLE_CHRONO_BASIS_VALUE";
        let ycol = "BASABLE_CHRONO_RESULT";

        // create query operation type
        let selection_columns = Some(vec![
            format!("{basis}({chrono_col}) as {xcol}"),
            format!("COUNT(*) as {ycol}"),
        ]);

        let operation = QueryOperation::SelectData(selection_columns);

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

        let order_by = Some(QueryOrder::DESC(xcol.to_string()));

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
