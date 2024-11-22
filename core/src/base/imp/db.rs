use uuid::Uuid;

use crate::base::data::table::{TableSearchOpts, TableSummaries};
use crate::base::query::filter::{Filter, FilterChain};
use crate::base::query::{BasableQuery, QueryCommand};
use crate::imp::database::mysql::db::MySqlDB;
use crate::imp::database::DbServerDetails;
use crate::AppError;

use super::graphs::VisualizeDB;
use super::{ConnectorType, SharedTable};

pub type DBQueryResult<R, E> = Result<Vec<R>, E>;

/// An abstraction of database connection.
pub trait DB: VisualizeDB + QuerySqlParser + Send + Sync {
    type Row;
    // type ColumnValue;

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
    fn query_tables(&self) -> DBQueryResult<Self::Row, AppError>;

    /// Get an instance of a [`SharedTable`], as a mutable thread-safe reference.
    fn get_table(&self, name: &str) -> Option<&SharedTable>;

    /// Get information about each table in the database and build a list from them.
    fn build_table_list(&self) -> Result<TableSummaries, AppError>;

    /// Details about the connection
    fn details(&self) -> Result<DbServerDetails, AppError>;

    /// Get total number of columns
    fn query_column_count(&self, table_name: &str) -> Result<u32, AppError>;
}

pub trait QuerySqlParser {
    fn parse_filter(filter: &Filter) -> String
    where
        Self: Sized,
    {
        filter.to_string()
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

    fn generate_sql(&self, query: BasableQuery) -> Result<String, AppError> {
        let is_search_mode = query.is_search_mode();

        let BasableQuery {
            table,
            command: operation,
            filters,
            row_count,
            offset,
            order_by,
            group_by,
            left_join,
            having,
            search_opts,
        } = query;

        // Parse query operation type
        let mut sql = match operation {
            QueryCommand::SelectData(cols) => {
                let select_cols = cols.map_or_else(
                    || "*".to_string(),
                    |list| {
                        if list.is_empty() {
                            return "*".to_string();
                        }

                        let s: Vec<String> = list
                            .iter()
                            .map(|s| {
                                if s.to_lowercase() == "count(*)" {
                                    return format!("{s}");
                                }

                                format!("`{s}`")
                            })
                            .collect();
                        s.join(", ")
                    },
                );

                format!("SELECT {select_cols} FROM {table}")
            }
        };

        // Parse left join
        if let Some(left_join) = left_join {
            sql.push_str(format!(" LEFT JOIN {left_join}").as_str())
        }

        // Parse query filters
        if filters.not_empty() && !is_search_mode {
            let filter_chain = <MySqlDB as QuerySqlParser>::parse_filter_chain(&filters);
            sql.push_str(format!(" WHERE {filter_chain}").as_str())
        }

        // parse fulltext search mode
        if is_search_mode {
            if let Some(opts) = search_opts {
                let TableSearchOpts {
                    search_cols, query, ..
                } = opts;

                let wrap_cols: Vec<String> =
                    search_cols.iter().map(|col| format!("`{col}`")).collect();

                let search_query = format!(
                    " WHERE MATCH({}) AGAINST('{}')",
                    wrap_cols.join(","),
                    query
                );

                sql.push_str(&search_query);
            }
        }

        // Parse GROUP BY
        if let Some(group_by) = group_by {
            let cols = group_by.join(", ");
            sql.push_str(format!(" GROUP BY {cols}").as_str());
        }

        // Parse HAVING
        if having.not_empty() {
            let filter_chain = <MySqlDB as QuerySqlParser>::parse_filter_chain(&having);
            sql.push_str(format!(" HAVING {filter_chain}").as_str())
        }

        // Parse ORDER BY
        if let Some(order) = order_by {
            sql.push_str(format!(" ORDER BY {order}").as_str());
        }

        // Parse LIMIT
        if let Some(row_count) = row_count {
            let offset = offset.unwrap_or_default();
            sql.push_str(format!(" LIMIT {offset}, {row_count}").as_str());
        }

        Ok(sql)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        base::query::{filter::FilterChain, BasableQuery, QueryCommand},
        tests::common::create_test_db,
        AppError,
    };

    #[test]
    fn test_generate_sql() -> Result<(), AppError> {
        let filters = FilterChain::new();

        let query = BasableQuery {
            table: "vhchartz".to_string(),
            command: QueryCommand::SelectData(None),
            filters,
            row_count: Some(100),
            ..Default::default()
        };

        let db = create_test_db()?;
        let sql = db.generate_sql(query);

        assert!(sql.is_ok());

        Ok(())
    }
}
