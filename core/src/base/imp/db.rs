use mysql::MySqlError;
use uuid::Uuid;

use crate::base::query::filter::{Filter, FilterChain, FilterCondition, FilterOperator};
use crate::base::query::{BasableQuery, QueryOperation};
use crate::base::{data::table::TableSummaries, AppError};
use crate::imp::database::mysql::db::MySqlDB;
use crate::imp::database::DbConnectionDetails;

use super::graphs::VisualizeDB;
use super::{ConnectorType, SharedTable};

pub type DBQueryResult<R, E> = Result<Vec<R>, E>;

pub type DBError = <MySqlDB as DB>::Error;

/// An abstraction of database connection.
pub trait DB: VisualizeDB + QuerySqlParser + Send + Sync {
    type Row;
    type Error;
    type ColumnValue;

    fn id(&self) -> &Uuid;

    fn user_id(&self) -> &str;

    /// Get the [`ConnectorType`] instance for [`DB`].
    fn connector(&self) -> &ConnectorType;

    /// Construct [`Table`](`crate::base::table::Table`) for all tables. This loads the constructed [`Table`](`crate::base::table::Table`) into `DB` instance.
    /// Caller should provide a [`ConnectorType`] pointer whose copy is assigned to each [Table](`crate::base::table::Table`) that is created.
    ///
    /// The [`ConnectorType`] will be used by the table for their own queries.
    fn load_tables(&mut self, connector: ConnectorType) -> Result<(), AppError>;

    fn tables(&self) -> &Vec<SharedTable>;

    /// Query [`DB`] server for information about available tables. It only queries the database server and
    /// return results as [`DB::Row`]. It is different from [`DB::load_tables`] which actually loads the [`Table`]
    /// abstraction into memory.
    fn query_tables(&self) -> DBQueryResult<Self::Row, Self::Error>;

    /// Get an instance of a [`SharedTable`], as a mutable thread-safe reference.
    fn get_table(&self, name: &str) -> Option<&SharedTable>;

    /// Query connection tables from DB source and return table summaries
    fn query_table_summaries(&self) -> Result<TableSummaries, AppError>;

    /// Details about the connection
    fn details(&self) -> Result<DbConnectionDetails, AppError>;

    /// Get total number of columns
    fn query_column_count(&self, table_name: &str) -> Result<u32, AppError>;
}

pub trait QuerySqlParser {
    fn parse_filter_operator(fo: &FilterOperator) -> String
    where
        Self: Sized,
    {
        match fo {
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
        }
    }

    fn parse_filter_condition(c: &FilterCondition) -> String
    where
        Self: Sized,
    {
        format!("{} {}", c.column, Self::parse_filter_operator(&c.operator))
    }

    fn parse_filter(filter: &Filter) -> String
    where
        Self: Sized,
    {
        match filter {
            Filter::BASE(c) => Self::parse_filter_condition(c),
            Filter::AND(c) => format!("AND {}", Self::parse_filter_condition(c)),
            Filter::OR(c) => format!("OR {}", Self::parse_filter_condition(c)),
        }
    }

    fn parse_filter_chain(filters: &FilterChain) -> String
    where
        Self: Sized,
    {
        let filters: Vec<String> = filters
            .all()
            .iter()
            .map(|f| Self::parse_filter(f))
            .collect();
        filters.join(" ")
    }

    fn generate_sql(&self, query: BasableQuery) -> Result<String, MySqlError> {
        let BasableQuery {
            table,
            operation,
            filters,
            limit,
            order_by,
            group_by
        } = query;

        // Parse query operation type
        let mut sql = match operation {
            QueryOperation::SelectData(cols) => {
                let mut select_cols = String::from("*");
                if let Some(col_list) = cols {
                    select_cols = col_list.join(", ");
                };

                format!("SELECT {select_cols} FROM {table}")
            }
        };

        // Parse query filters
        if filters.not_empty() {
            let filter_chain = <MySqlDB as QuerySqlParser>::parse_filter_chain(&filters);
            sql.push_str(format!(" WHERE {filter_chain}").as_str())
        }

        // Parse GROUP BY
        if let Some(group_by) = group_by {
            let cols = group_by.join(", ");
            sql.push_str(format!(" GROUP BY {cols}").as_str());
        }

        // Parse ORDER BY
        if let Some(order) = order_by {
            sql.push_str(format!(" ORDER BY {order}").as_str());
        }

        // Parse LIMIT
        if let Some(limit) = limit {
            sql.push_str(format!(" LIMIT {limit}").as_str());
        }

        Ok(sql)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        base::{
            query::{
                filter::{Filter, FilterChain, FilterCondition, FilterOperator},
                BasableQuery, QueryOperation,
            },
            AppError,
        },
        tests::common::create_test_db,
    };

    #[test]
    fn test_generate_sql() -> Result<(), AppError> {
        let mut filters = FilterChain::new();

        let c1 = FilterCondition {
            column: "publisher".to_string(),
            operator: FilterOperator::Eq("Rockstar Games".to_string()),
        };

        let c2 = FilterCondition {
            column: "release_date".to_string(),
            operator: FilterOperator::Btw("2010-09-01".to_string(), "2010-11-30".to_string()),
        };

        filters.add_multiple(vec![Filter::BASE(c1), Filter::AND(c2)]);

        let query = BasableQuery {
            table: "vhchartz".to_string(),
            operation: QueryOperation::SelectData(None),
            filters,
            limit: Some(100),
            ..Default::default()
        };

        let db = create_test_db()?;
        let sql = db.generate_sql(query);

        assert!(sql.is_ok());

        Ok(())
    }
}
