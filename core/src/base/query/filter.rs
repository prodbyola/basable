use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::globals::QUERY_FILTER_PREFIX;

#[derive(Clone, Default)]
pub enum FilterOperator {
    Eq(String),
    NotEq(String),
    Gt(String),
    Lt(String),
    Gte(String),
    Lte(String),
    Like(String),
    NotLike(String),
    LikeSingle(String),
    NotLikeSingle(String),
    Regex(String),
    NotRegex(String),
    Btw(String, String),
    NotBtw(String, String),
    Contains(Vec<String>),
    NotContains(Vec<String>),

    #[default]
    Null,

    NotNull,
}

impl Display for FilterOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            FilterOperator::Eq(v) => format!("= '{v}'"),
            FilterOperator::NotEq(v) => format!("!= '{v}'"),
            FilterOperator::Gt(v) => format!("> '{v}'"),
            FilterOperator::Lt(v) => format!("< '{v}'"),
            FilterOperator::Gte(v) => format!(">= '{v}'"),
            FilterOperator::Lte(v) => format!("<= '{v}'"),
            FilterOperator::Like(v) => format!("LIKE '{v}%'"),
            FilterOperator::NotLike(v) => format!("NOT LIKE '{v}%'"),
            FilterOperator::LikeSingle(v) => format!("LIKE '_{v}%'"),
            FilterOperator::NotLikeSingle(v) => format!("NOT LIKE '_{v}%'"),
            FilterOperator::Regex(v) => format!("REGEXP '{v}'"),
            FilterOperator::NotRegex(v) => format!("NOT REGEXP '{v}'"),
            FilterOperator::Btw(start, end) => format!("BETWEEN '{start}' AND '{end}'"),
            FilterOperator::NotBtw(start, end) => format!("NOT BETWEEN '{start}' AND '{end}'"),
            FilterOperator::Contains(values) => {
                let v: Vec<String> = values.iter().map(|v| v.to_string()).collect();
                let v = v.join(", ");

                format!("IN ({v})")
            }
            FilterOperator::NotContains(values) => {
                let v: Vec<String> = values.iter().map(|v| v.to_string()).collect();
                let v = v.join(", ");

                format!("NOT IN ({v})")
            }
            FilterOperator::Null => "IS NULL".to_string(),
            FilterOperator::NotNull => "IS NOT NULL".to_string(),
        };

        write!(f, "{}", op)
    }
}

/// [FilterComparator] is useful for filtering query columns by comparing the value of
/// [FilterComparator::column] to the value given to the [FilterComparator::operator]. Please
/// see [FilterOperator] for different comparison operations.
#[derive(Clone, Default)]
pub struct FilterComparator {
    pub column: String,
    pub operator: FilterOperator,
}

impl Display for FilterComparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.column, self.operator)
    }
}

#[derive(Clone, EnumIter)]
pub enum Filter {
    BASE(FilterComparator),
    AND(FilterComparator),
    OR(FilterComparator),
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filter = match self {
            Filter::BASE(c) => c.to_string(),
            Filter::AND(c) => format!("AND {c}"),
            Filter::OR(c) => format!("OR {c}"),
        };

        write!(f, "{QUERY_FILTER_PREFIX}{filter}")
    }
}

impl TryFrom<String> for Filter {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for filter in Filter::iter() {
            if filter.to_string() == value {
                return Ok(filter);
            }
        }

        Err("Error converting type to Filter".to_string())
    }
}

#[derive(Default)]
pub struct FilterChain(Vec<Filter>);
impl FilterChain {
    pub fn new() -> Self {
        FilterChain(Vec::new())
    }

    pub fn add_one(&mut self, filter: Filter) {
        self.0.push(filter);
    }

    pub fn add_multiple(&mut self, filters: Vec<Filter>) {
        filters.iter().for_each(|f| self.0.push(f.clone()));
    }

    pub fn all(&self) -> &Vec<Filter> {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn not_empty(&self) -> bool {
        !self.is_empty()
    }
}

impl Display for FilterChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values = &self.0;

        if !values.is_empty() {
            let first = values.get(0).unwrap();

            if !matches!(first, &Filter::BASE(_)) {
                return Err(std::fmt::Error);
            }
        }

        let values: Vec<String> = values.iter().map(|f| f.to_string()).collect();
        let values = values.join(",");

        write!(f, "{values}")
    }
}
