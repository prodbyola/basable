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