use std::sync::Arc;

use connector::Connector;
use db::DB;
use table::Table;

use crate::imp::database::mysql::{connector::MysqlConnector, db::MySqlDB, table::MySqlTable};

pub(crate) mod connector;
pub(crate) mod db;
pub(crate) mod graphs;
pub(crate) mod table;

/// Dynamic [`DB`] type to be implemented across the app.
pub(crate) type DbType = dyn DB<
    Row = <MySqlDB as DB>::Row,
    // ColumnValue = <MySqlDB as DB>::ColumnValue,
>;

/// Dynamic [`Connector`] type implemented across the app.
pub(crate) type ConnectorType = Arc<dyn Connector<Row = <MysqlConnector as Connector>::Row>>;

/// Dynamic [`Table`] type implemented across the app.
pub(crate) type TableType = dyn Table<
    Row = <MySqlTable as Table>::Row,
>;

/// A thread-safe sharable DB instance
pub(crate) type SharedDB = Arc<DbType>;

/// A thread-safe sharable Database Table
pub(crate) type SharedTable = Arc<TableType>;
