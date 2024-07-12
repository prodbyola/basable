use std::fmt::Display;

#[derive(Clone)]
pub(crate) enum ChronoAnalysisBasis {
    Daily,
    Monthly,
    Yearly,
}

impl ChronoAnalysisBasis {
    fn values() -> [Self; 3] {
        [ChronoAnalysisBasis::Daily, ChronoAnalysisBasis::Monthly, ChronoAnalysisBasis::Yearly]
    }
}

impl From<ChronoAnalysisBasis> for String {
    fn from(value: ChronoAnalysisBasis) -> Self {
        let basis = match value {
            ChronoAnalysisBasis::Daily => "Date",
            ChronoAnalysisBasis::Monthly => "Month",
            ChronoAnalysisBasis::Yearly => "Year",
        };

        basis.to_string()
    }
}

impl TryFrom<String> for ChronoAnalysisBasis {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for b in ChronoAnalysisBasis::values() {
            let test = String::from(b.clone());
            if value == test {
                return Ok(b)
            }
        }

        Err("error parsing analysis basis".to_string())
    }
}


impl Display for ChronoAnalysisBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let basis: String = self.clone().into();
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

impl TryFrom<String> for ChronoAnalysisRange {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let spl: Vec<&str> = value.split("-").collect();
        if spl.len() == 2 {
            let start = spl.get(0).unwrap();
            let end = spl.get(1).unwrap();

            let range = ChronoAnalysisRange(start.trim().to_string(), end.trim().to_string());
            return Ok(range)
        }

        Err("error parsing analysis range".to_string())
    }
}

pub(crate) struct ChronoAnalysisOpts {
    pub table: String,
    pub chrono_col: String,
    pub basis: ChronoAnalysisBasis,
    pub range: ChronoAnalysisRange,
}
