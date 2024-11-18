use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub enum FilterExpression {
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

impl Display for FilterExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            FilterExpression::Eq(v) => format!("= `{v}`"),
            FilterExpression::NotEq(v) => format!("!= `{v}`"),
            FilterExpression::Gt(v) => format!("> `{v}`"),
            FilterExpression::Lt(v) => format!("< `{v}`"),
            FilterExpression::Gte(v) => format!(">= `{v}`"),
            FilterExpression::Lte(v) => format!("<= `{v}`"),
            FilterExpression::Like(v) => format!("LIKE `{v}%`"),
            FilterExpression::NotLike(v) => format!("NOT LIKE `{v}%`"),
            FilterExpression::LikeSingle(v) => format!("LIKE `_{v}%`"),
            FilterExpression::NotLikeSingle(v) => format!("NOT LIKE `_{v}%`"),
            FilterExpression::Regex(v) => format!("REGEXP `{v}`"),
            FilterExpression::NotRegex(v) => format!("NOT REGEXP `{v}`"),
            FilterExpression::Btw(start, end) => format!("BETWEEN (`{start}` AND `{end}`)"),
            FilterExpression::NotBtw(start, end) => format!("NOT BETWEEN (`{start}` AND `{end}`)"),
            FilterExpression::Contains(values) => {
                let v: Vec<String> = values.iter().map(|v| v.to_string()).collect();
                let v = v.join(", ");

                format!("IN ({v})")
            }
            FilterExpression::NotContains(values) => {
                let v: Vec<String> = values.iter().map(|v| v.to_string()).collect();
                let v = v.join(", ");

                format!("NOT IN ({v})")
            }
            FilterExpression::Null => "IS NULL".to_string(),
            FilterExpression::NotNull => "IS NOT NULL".to_string(),
        };

        write!(f, "{}", op)
    }
}

// /// [FilterPredicate] is useful for filtering query columns by comparing the value of
// /// [FilterPredicate::column] to the value given to the [FilterPredicate::operator]. Please
// /// see [FilterOperator] for different comparison operations.
// #[derive(Clone, Default)]
// pub struct FilterPredicate {
//     pub column: String,
//     pub operator: FilterExpression,
// }

// impl Display for FilterPredicate {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "`{}` {}", self.column, self.operator)
//     }
// }

#[derive(Deserialize, Serialize)]
pub enum FilterCombinator {
    BASE, AND, OR
}

#[derive(Deserialize, Serialize)]
pub struct Filter {
    pub combinator: FilterCombinator,
    pub column: String,
    pub expression: FilterExpression
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let comb = match self.combinator {
            FilterCombinator::AND => "AND ",
            FilterCombinator::OR => "OR ",
            FilterCombinator::BASE => ""
        };

        write!(f, "{comb} {} {}", self.column, self.expression)
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

    // pub fn add_multiple(&mut self, filters: Vec<Filter>) {
    //     filters.iter().for_each(|f| self.0.push(f.clone()));
    // }

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
            if let Some(first) = values.get(0) {
                if !matches!(&first.combinator, &FilterCombinator::BASE) {
                    return Err(std::fmt::Error);
                }
            }
        }

        let values: Vec<String> = values.iter().map(|f| f.to_string()).collect();
        let values = values.join(",");

        write!(f, "{values}")
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::Filter;

    #[test]
    pub fn test_serialize_filter() {
        let filter = Filter {
            combinator: super::FilterCombinator::BASE,
            column: "test_column".to_string(),
            expression: super::FilterExpression::Gte("310".to_string())
        };

        let s = serde_json::to_string(&filter).unwrap();
        println!("{s}")
    }
}
