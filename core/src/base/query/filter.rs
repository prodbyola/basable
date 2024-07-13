use std::fmt::Display;

#[derive(Clone)]
pub enum FilterOperator<V: Display + Clone> {
    Eq(V),
    NotEq(V),
    Gt(V),
    Lt(V),
    Gte(V),
    Lte(V),
    Like(V),
    NotLike(V),
    LikeSingle(V),
    NotLikeSingle(V),
    Regex(V),
    NotRegex(V),
    Btw(V, V),
    NotBtw(V, V),
    Contains(Vec<V>),
    NotContains(Vec<V>),
    Null,
    NotNull,
}

impl<V: Display + Clone> Display for FilterOperator<V> {
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

#[derive(Clone)]
pub struct FilterCondition<V: Display + Clone> {
    pub column: String,
    pub operator: FilterOperator<V>,
}

impl<V: Display + Clone> Display for FilterCondition<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.column, self.operator)
    }
}

#[derive(Clone)]
pub enum Filter<V: Display + Clone> {
    BASE(FilterCondition<V>),
    AND(FilterCondition<V>),
    OR(FilterCondition<V>),
}

impl<V: Display + Clone> Display for Filter<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filter = match self {
            Filter::BASE(c) => c.to_string(),
            Filter::AND(c) => format!("AND {c}"),
            Filter::OR(c) => format!("OR {c}"),
        };

        write!(f, "{}", filter)
    }
}

pub struct FilterChain<V: Display + Clone>(Vec<Filter<V>>);
impl<V: Display + Clone> FilterChain<V> {
    pub fn new() -> Self {
        FilterChain(Vec::new())
    }

    pub fn add_filter(&mut self, filter: Filter<V>) {
        self.0.push(filter);
    }

    pub fn add_filters(&mut self, filters: Vec<Filter<V>>) {
        filters.iter().for_each(|f| self.0.push(f.clone()));
    }
}

impl<V: Display + Clone> Display for FilterChain<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values = &self.0;

        if !values.is_empty() {
            let first = values.get(0).unwrap();

            if !matches!(first, &Filter::BASE(_)) {
                return Err(std::fmt::Error)
            }
        }

        let values: Vec<String> = values.iter().map(|f| f.to_string()).collect();
        let values = values.join(" ");

        write!(f, "{values}")
    }
}
