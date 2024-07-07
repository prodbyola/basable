use std::collections::HashMap;

use crate::{base::{column::ColumnList, data::table::{DataQueryFilter, DataQueryResult, TableConfig, UpdateDataOptions}}, imp::database::mysql::table::MySqlTable};

use super::ConnectorType;

pub(crate) type TableError = <MySqlTable as Table>::Error;
pub(crate) type TableColumn = <MySqlTable as Table>::ColumnValue;

pub(crate) trait Table: TableCRUD + Sync + Send {
    type Error;
    type Row;
    type ColumnValue;

    /// Create a new [`Table`] and assign the given [`ConnectorType`].
    ///
    /// It returns the new [`Table`]. And if a [`TableConfig`] is created for the table,
    /// then the config is also returned. It is up to the caller to save or send the config for the table.
    ///
    /// # Example:
    /// ```
    /// let (table, config) = Table::new("table_name".to_string(), conn);
    /// // config is Option<TableConfig>
    /// ```
    ///
    /// This call initializes [`TableConfig`] for the table if certain query conditions are true for the table.
    /// For example if the table has a column named id, a primary key or a unique column, we automatically
    /// set the `pk` field of the table to any of the column.
    fn new(name: String, conn: ConnectorType) -> Self
    where
        Self: Sized;

    /// [Table]'s name
    fn name(&self) -> &str;

    /// Get the table's [`ConnectorType`].
    fn connector(&self) -> &ConnectorType;

    /// Retrieve all columns for the table
    fn query_columns(&self) -> Result<ColumnList, <Self as Table>::Error>;

    fn init_config(&self) -> Option<TableConfig>;
}

pub(crate) trait TableCRUD {
    /// Inserts a new data into the table.
    fn insert_data(&self, input: HashMap<String, String>) -> Result<(), TableError>;

    /// Retrieve data from table based on query `filter`.
    fn query_data(
        &self,
        filter: DataQueryFilter,
    ) -> DataQueryResult<TableColumn, TableError>;

    fn update_data(&self, input: UpdateDataOptions) -> Result<(), TableError>;

    fn delete_data(&self, col: String, value: String) -> Result<(), TableError>;
}

#[cfg(test)]
mod tests {

    use crate::{
        base::{imp::table::DataQueryFilter, AppError},
        tests::common::{create_test_db, get_test_db_table},
    };

    #[test]
    fn test_table_query_column() -> Result<(), AppError> {
        let db = create_test_db()?;
        let table_name = get_test_db_table();

        assert!(db.get_table(&table_name).is_some());

        if let Some(table) = db.get_table("swp") {
            let cols = table.query_columns();

            assert!(cols.is_ok());
        }

        Ok(())
    }

    #[test]
    fn test_table_query_data() -> Result<(), AppError> {
        let db = create_test_db()?;
        let table_name = get_test_db_table();

        if let Some(table) = db.get_table(&table_name) {
            let filter = DataQueryFilter::default();
            let data = table.query_data(filter);
            assert!(data.is_ok());
        }

        Ok(())
    }
}

#[cfg(test)]
mod interactive_tests {
    use std::{collections::HashMap, io::stdin};

    use crate::{
        base::{imp::table::UpdateDataOptions, AppError},
        tests::common::{create_test_db, get_test_db_table},
    };

    #[test]
    fn test_table_insert_data() -> Result<(), AppError> {
        let db = create_test_db()?;
        let table_name = get_test_db_table();

        if let Some(table) = db.get_table(&table_name) {
            let mut test_data = HashMap::new();
            let quit_word = "cont";

            println!(
                "
                Let's add some data into our TEST_DB_TABLE_NAME. \n
                Please enter your data inputs in the format: column,value. \n
                Enter '{}' to continue the test.
            ",
                quit_word
            );

            loop {
                let mut input = String::new();
                println!("Please enter an input:");
                stdin()
                    .read_line(&mut input)
                    .expect("Please enter a valid string");

                let input = input.trim().to_string();
                if input == quit_word {
                    break;
                }

                let spl: Vec<&str> = input.split(",").collect();
                test_data.insert(spl[0].to_string(), spl[1].to_string());
            }

            let insert_data = table.insert_data(test_data);

            assert!(insert_data.is_ok());
        }

        Ok(())
    }

    #[test]
    fn test_table_update_data() -> Result<(), AppError> {
        let db = create_test_db()?;
        let table_name = get_test_db_table();

        if let Some(table) = db.get_table(&table_name) {
            let mut test_data = UpdateDataOptions::default();

            // Get update clause
            println!("Please enter update clause as key,value");
            let mut input = String::new();

            stdin()
                .read_line(&mut input)
                .expect("Please enter a valid string");

            let input = input.trim().to_string();

            let spl: Vec<&str> = input.split(",").collect();
            test_data.key = spl[0].to_string();
            test_data.value = spl[1].to_string();

            // Get update value
            println!("Please enter update clause as column,value");
            let mut input = String::new();

            stdin()
                .read_line(&mut input)
                .expect("Please enter a valid string");

            let input = input.trim().to_string();

            let spl: Vec<&str> = input.split(",").collect();
            test_data
                .input
                .insert(spl[0].to_string(), spl[1].to_string());

            // update the table
            let update_data = table.update_data(test_data);

            assert!(update_data.is_ok());
        }

        Ok(())
    }
}
