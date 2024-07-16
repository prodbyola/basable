use std::fmt::Display;

use filter::FilterChain;

pub mod filter;

pub enum QueryOperation {
    SelectData(Option<Vec<String>>),
}

impl Default for QueryOperation {
    fn default() -> Self {
        Self::SelectData(None)
    }
}

pub enum QueryOrder {
    ASC(String),
    DESC(String),
}

impl Display for QueryOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let order = match self {
            QueryOrder::ASC(col) => format!("{col} ASC"),
            QueryOrder::DESC(col) => format!("{col} DESC"),
        };

        write!(f, "{order}")
    }
}

#[derive(Default)]
pub struct BasableQuery {
    pub table: String,
    pub operation: QueryOperation,
    pub filters: FilterChain,
    pub limit: Option<usize>,
    pub order_by: Option<QueryOrder>,
    pub group_by: Option<Vec<String>>,
    pub left_join: Option<String>,
    pub having: FilterChain,
}
