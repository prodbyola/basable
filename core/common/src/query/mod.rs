use std::fmt::Display;

use filter::FilterChain;
use serde::Deserialize;

use super::data::table::TableSearchOpts;

pub mod filter;

pub enum QueryCommand {
    SelectData(Option<Vec<String>>),
}

impl Default for QueryCommand {
    fn default() -> Self {
        Self::SelectData(None)
    }
}

#[derive(Deserialize)]
pub enum QueryOrder {
    ASC(String),
    DESC(String),
}

impl Display for QueryOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let order = match self {
            QueryOrder::ASC(col) => format!("`{col}` ASC"),
            QueryOrder::DESC(col) => format!("`{col}` DESC"),
        };

        write!(f, "{order}")
    }
}

#[derive(Default)]
pub struct BasableQuery {
    pub table: String,
    pub command: QueryCommand,
    pub filters: FilterChain,
    pub row_count: Option<usize>,
    pub offset: Option<usize>,
    pub order_by: Option<QueryOrder>,
    pub group_by: Option<Vec<String>>,
    pub left_join: Option<String>,
    pub having: FilterChain,
    pub search_opts: Option<TableSearchOpts>
}

impl BasableQuery {
    pub fn is_search_mode(&self) -> bool {
        self.search_opts.is_some()
    }
}
