use std::sync::Arc;

use connector::Connector;
use db::DB;
use table::Table;

use crate::imp::database::mysql::{connector::MysqlConnector, db::MySqlDB, table::MySqlTable};

pub(crate) mod connector;
pub(crate) mod db;
pub(crate) mod table;

/// Dynamic [`DB`] type implemented across the app.
pub(crate) type DbType = dyn DB<
    Row = <MySqlDB as DB>::Row,
    Error = <MySqlDB as DB>::Error,
    ColumnValue = <MySqlDB as DB>::ColumnValue,
>;

/// Dynamic [`Connector`] type implemented across the app.
pub(crate) type ConnectorType = Arc<
    dyn Connector<
        Row = <MysqlConnector as Connector>::Row,
        Error = <MysqlConnector as Connector>::Error,
    >,
>;

/// Dynamic [`Table`] type implemented across the app.
pub(crate) type TableType = dyn Table<
    Row = <MySqlTable as Table>::Row,
    Error = <MySqlTable as Table>::Error,
    ColumnValue = <MySqlTable as Table>::ColumnValue,
>;

pub(crate) type SharedDB = Arc<DbType>;