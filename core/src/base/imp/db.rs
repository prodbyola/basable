use uuid::Uuid;

use crate::base::{data::table::TableSummaries, AppError};
use crate::imp::database::mysql::db::MySqlDB;
use crate::imp::database::DbConnectionDetails;

use super::{ ConnectorType, SharedTable};

pub(crate) type DBQueryResult<R, E> = Result<Vec<R>, E>;

pub(crate) type DBError = <MySqlDB as DB>::Error;

/// An abstraction of database connection.
pub(crate) trait DB: AnalyzeDB + Send + Sync {
    type Row;
    type Error;
    type ColumnValue;

    fn id(&self) -> &Uuid;

    fn user_id(&self) -> &str;

    /// Get the [`ConnectorType`] instance for [`DB`].
    fn connector(&self) -> &ConnectorType;

    /// Create [`Table`](`crate::base::table::Table`) for all tables and load them into `DB` instance. Caller should provide a [`ConnectorType`]
    /// pointer whose copy is assigned to each [Table](`crate::base::table::Table`) that is created.
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

pub(crate) enum ChronoAnalysisBasis {
    Daily,
    Monthly,
    Yearly
}

pub (crate) struct ChronoAnalysisRange(pub String, pub String);
pub(crate) struct ChronoAnalysisOpts {
    pub table: String,
    pub chrono_col: String,
    pub basis: ChronoAnalysisBasis,
    pub range: ChronoAnalysisRange
}

pub(crate) trait AnalyzeDB {
    fn chrono_analysis(&self, opts: ChronoAnalysisOpts) -> Result<(), DBError>;
}

#[cfg(test)]
mod tests {
    use crate::{base::{imp::db::ChronoAnalysisRange, AppError}, tests::common::create_test_db};

    use super::ChronoAnalysisOpts;

    #[test]
    fn test_chrono_analysis() -> Result<(), AppError>{
        let db = create_test_db()?;
        let analyze = db.chrono_analysis(ChronoAnalysisOpts{
            table: "vgchartz".to_string(),
            chrono_col: "release_date".to_string(),
            basis: super::ChronoAnalysisBasis::Daily,
            range: ChronoAnalysisRange("2010-11-01".to_string(), "2010-11-30".to_string())
        });

        assert!(analyze.is_ok());

        Ok(())
    }
}