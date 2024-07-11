use std::fmt::Display;

pub enum TrendAnalysisType { IntraModel, CrossModel }
pub enum TrendAnalysisOrder { DESC, ASC }

impl Display for TrendAnalysisOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let order = match self {
            TrendAnalysisOrder::DESC => "DESC",
            TrendAnalysisOrder::ASC => "ASC",
        };

        write!(f, "{}", order)
    }
}

pub struct TrendAnalysisOpts {
    pub table: String,
    pub analysis_type: TrendAnalysisType,
    pub xcol: String,
    pub ycol: String,
    pub order: TrendAnalysisOrder,
    pub limit: usize
}