use std::fmt::Display;

use serde::{Deserialize, Serialize};

fn escape_special_characters(input: &str) -> String {
    input
        .replace('\\', "\\\\") // Escape backslashes
        .replace('"', "\\\"") // Escape double quotes
        .replace('\'', "\\\'") // Escape single quotes
        .replace('\n', "\\n") // Escape newline
        .replace('\t', "\\t") // Escape tab
        .replace('\r', "\\r") // Escape carriage return
        .replace('&', "\\&") // Escape ampersand
        .replace('_', "\\_") // Escape underscore
}

#[derive(Deserialize, Serialize, Default)]
pub enum FilterExpression {
    Eq(String),
    NotEq(String),
    Gt(String),
    Lt(String),
    Gte(String),
    Lte(String),
    Contains(String),
    NotContains(String),
    // Like(String),
    // NotLike(String),
    // LikeSingle(String),
    // NotLikeSingle(String),
    Regex(String),
    NotRegex(String),
    Btw(String, String),
    NotBtw(String, String),
    Includes(Vec<String>),
    NotInclude(Vec<String>),

    #[default]
    Null,

    NotNull,
}

impl Display for FilterExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            FilterExpression::Eq(v) => format!("= '{}'", escape_special_characters(v)),
            FilterExpression::NotEq(v) => format!("!= '{}'", escape_special_characters(v)),
            FilterExpression::Gt(v) => format!("> '{}'", escape_special_characters(v)),
            FilterExpression::Lt(v) => format!("< '{}'", escape_special_characters(v)),
            FilterExpression::Gte(v) => format!(">= '{}'", escape_special_characters(v)),
            FilterExpression::Lte(v) => format!("<= '{}'", escape_special_characters(v)),
            FilterExpression::Contains(v) => format!("LIKE '%{}%'", escape_special_characters(v)),
            FilterExpression::NotContains(v) => format!("NOT REGEXP '%{}%'", escape_special_characters(v)),
            // FilterExpression::Like(v) => format!("LIKE '{}%'", escape_special_characters(v)),
            // FilterExpression::NotLike(v) => format!("NOT LIKE '{}%'", escape_special_characters(v)),
            // FilterExpression::LikeSingle(v) => format!("LIKE '_{}%'", escape_special_characters(v)),
            // FilterExpression::NotLikeSingle(v) => format!("NOT LIKE '_{}%'", escape_special_characters(v)),
            FilterExpression::Regex(v) => format!("REGEXP '{}'", escape_special_characters(v)),
            FilterExpression::NotRegex(v) => format!("NOT REGEXP '{}'", escape_special_characters(v)),
            FilterExpression::Btw(start, end) => format!("BETWEEN ('{}' AND '{}')", escape_special_characters(start), escape_special_characters(end)),
            FilterExpression::NotBtw(start, end) => format!("NOT BETWEEN ('{}' AND '{}')", escape_special_characters(start), escape_special_characters(end)),
            FilterExpression::Includes(values) => {
                let v: Vec<String> = values.iter().map(|v| escape_special_characters(v)).collect();
                let v = v.join(", ");

                format!("IN ({v})")
            }
            FilterExpression::NotInclude(values) => {
                let v: Vec<String> = values.iter().map(|v| escape_special_characters(v)).collect();
                let v = v.join(", ");

                format!("NOT IN ({v})")
            }
            FilterExpression::Null => "IS NULL".to_string(),
            FilterExpression::NotNull => "IS NOT NULL".to_string(),
        };

        write!(f, "{}", op)
    }
}

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

        write!(f, "{comb} `{}` {}", self.column, self.expression)
    }
}

#[derive(Default)]
pub struct FilterChain(Vec<Filter>);
impl FilterChain {
    pub fn new() -> Self {
        FilterChain(Vec::new())
    }

    pub fn empty() -> FilterChain {
        FilterChain(Vec::with_capacity(0))
    }
    
    pub fn prefill(filters: Vec<Filter>) -> FilterChain {
        FilterChain(filters)
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
