use time::Date;

use crate::{base::{imp::{db::{DBError, QuerySqlParser, DB}, graphs::{category::CategoryGraphOpts, chrono::{ChronoAnalysisBasis, ChronoAnalysisOpts}, trend::{TrendAnalysisOpts, TrendAnalysisType}, AnalysisResult, AnalysisResults, AnalysisValue, VisualizeDB}}, AppError}, globals::{BASABLE_CHRONO_XCOL, BASABLE_CHRONO_YCOL}};

use super::db::MySqlDB;
use mysql::{DriverError::SetupError, Value};

impl VisualizeDB for MySqlDB {
    fn chrono_graph(&self, opts: ChronoAnalysisOpts) -> Result<AnalysisResults, DBError> {
        let basis = opts.basis.clone();

        let query = opts.into();
        let sql = self.generate_sql(query)?;

        let conn = self.connector();
        let rows = conn.exec_query(&sql)?;

        let results: AnalysisResults = rows
            .iter()
            .map(|r| {
                let x = match basis {
                    ChronoAnalysisBasis::Daily => {
                        let date: Date = r.get(BASABLE_CHRONO_XCOL).unwrap();
                        AnalysisValue::Date(date)
                    }
                    _ => AnalysisValue::UInt(r.get(BASABLE_CHRONO_XCOL).unwrap()),
                };

                let y = AnalysisValue::UInt(r.get(BASABLE_CHRONO_YCOL).unwrap());

                AnalysisResult::new(x, y)
            })
            .collect();

        Ok(results)
    }

    fn trend_graph(&self, opts: TrendAnalysisOpts) -> Result<AnalysisResults, DBError> {
        
        let xcol = opts.xcol.clone();
        let ycol = opts.ycol.clone();
        let analysis_type = opts.analysis_type.clone();
        
        let query = opts
            .try_into()
            .map_err(|_| mysql::Error::DriverError(SetupError));

        let query = query?;
        let sql = self.generate_sql(query)?;

        let conn = self.connector();
        let rows = conn.exec_query(&sql)?;

        let results: AnalysisResults = rows
            .iter()
            .map(|r| {
                let x = AnalysisValue::Text(r.get(xcol.as_str()).unwrap());
                let y = match analysis_type {
                    TrendAnalysisType::IntraModel => {
                        AnalysisValue::Double(r.get(ycol.as_str()).unwrap())
                    }
                    TrendAnalysisType::CrossModel => {
                        AnalysisValue::UInt(r.get(ycol.as_str()).unwrap())
                    }
                };

                AnalysisResult::new(x, y)
            })
            .collect();

        Ok(results)
    }

    fn category_graph(&self, opts: CategoryGraphOpts) -> Result<AnalysisResults, AppError> {
        let CategoryGraphOpts {
            table,
            graph_type,
            target_col,
            limit,
        } = opts;
        let query = format!(
            "
                SELECT COUNT(*) as COUNT, {target_col}
                FROM {table}
                GROUP BY {target_col}
                LIMIT {limit}
            "
        );

        let conn = self.connector();

        let rows = conn.exec_query(&query)?;
        let results: AnalysisResults = rows.iter().map(|r| {
            let x = AnalysisValue::UInt(r.get("COUNT").unwrap());

            let y_value: Value = r.get(target_col.as_str()).unwrap();
            let y = y_value.into();

            AnalysisResult::new(x, y)
        }).collect();

        Ok(results)
    }
}
