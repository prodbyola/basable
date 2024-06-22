#[cfg(test)]
mod routes {
    use axum::{
        extract::{Path, State},
        Json,
    };

    use crate::{
        base::{table::TableConfig, AppError},
        http::routes::table::{get_columns, get_configuration, save_configuration},
        tests::common::{create_test_auth_extractor, create_test_state, get_test_db_table},
    };

    #[tokio::test]
    async fn test_save_table_config() -> Result<(), AppError> {
        let state = create_test_state(true)?;
        let auth_extractor = create_test_auth_extractor();
        let config = TableConfig::default();
        let table_name = get_test_db_table();

        let save_config =
            save_configuration(Path(table_name), auth_extractor, State(state), Json(config)).await;

        assert!(save_config.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_table_config() -> Result<(), AppError> {
        let state = create_test_state(true)?;
        let auth_extractor = create_test_auth_extractor();
        let table_name = get_test_db_table();

        let get_config = get_configuration(Path(table_name), auth_extractor, State(state)).await;

        assert!(get_config.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_table_columns() -> Result<(), AppError> {
        let state = create_test_state(true)?;
        let auth_extractor = create_test_auth_extractor();
        let table_name = get_test_db_table();

        let get_cols = get_columns(Path(table_name), auth_extractor, State(state)).await;

        assert!(get_cols.is_ok());
        Ok(())
    }
}
