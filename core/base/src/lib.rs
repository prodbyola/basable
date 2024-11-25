use std::sync::Arc;

use connector::Connector;
use db::DB;
use table::Table;

use mysql::{connector::MysqlConnector, db::MySqlDB, table::MySqlTable};


pub mod db;
pub mod graphs;
pub mod connector;
pub mod table;
pub mod config;
mod globals;

// we need to find a way to seperate mysql as an independent plugin
pub mod mysql;

/// Dynamic [`DB`] type to be implemented across the app.
pub type DbType = dyn DB<
    Row = <MySqlDB as DB>::Row,
>;

/// Dynamic [`Connector`] type implemented across the app.
pub type ConnectorType = Arc<dyn Connector<Row = <MysqlConnector as Connector>::Row>>;

/// Dynamic [`Table`] type implemented across the app.
pub type TableType = dyn Table<
    Row = <MySqlTable as Table>::Row,
>;

/// A thread-safe sharable DB instance
pub type SharedDB = Arc<DbType>;

/// A thread-safe sharable Database Table
pub type SharedTable = Arc<TableType>;

