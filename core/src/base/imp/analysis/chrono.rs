use std::fmt::Display;

pub(crate) enum ChronoAnalysisBasis {
    Daily,
    Monthly,
    Yearly,
}


impl Display for ChronoAnalysisBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let basis = match self {
            ChronoAnalysisBasis::Daily => "Date",
            ChronoAnalysisBasis::Monthly => "Month",
            ChronoAnalysisBasis::Yearly => "Year",
        };

        
        write!(f, "{}", basis)
    }
}

pub(crate) struct ChronoAnalysisRange(pub String, pub String);
impl ChronoAnalysisRange {
    pub fn start(&self) -> &str {
        &self.0
    }

    pub fn end(&self) -> &str {
        &self.1
    }
}

pub(crate) struct ChronoAnalysisOpts {
    pub table: String,
    pub chrono_col: String,
    pub basis: ChronoAnalysisBasis,
    pub range: ChronoAnalysisRange,
}
