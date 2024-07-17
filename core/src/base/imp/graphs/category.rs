use crate::base::query::{BasableQuery, QueryOperation};

pub enum CategoryGraphType {
    Simple,
    ManyToMany,
    Manual
}

pub struct CategoryGraphOpts {
    pub table: String,
    pub graph_type: CategoryGraphType,
    pub target_col: String,
    pub limit: usize
}

impl From<CategoryGraphOpts> for BasableQuery {
    fn from(value: CategoryGraphOpts) -> Self {
        let CategoryGraphOpts {
            table,
            graph_type,
            target_col,
            limit,
        } = value;
        
        let select_columns = vec!["COUNT(*) as COUNT".to_string(), target_col.clone()];
        let operation = QueryOperation::SelectData(Some(select_columns));

        BasableQuery {
            table,
            operation,
            group_by: Some(vec![target_col]),
            limit: Some(limit),
            ..Default::default()
        }
    }
}