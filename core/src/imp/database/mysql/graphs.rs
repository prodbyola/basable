use time::Date;

use crate::base::{imp::{db::{DBError, DB}, graphs::{category::CategoryGraphOpts, chrono::{ChronoAnalysisBasis, ChronoAnalysisOpts}, trend::{TrendAnalysisOpts, TrendAnalysisType}, AnalysisResult, AnalysisResults, AnalysisValue, VisualizeDB}}, AppError};

use super::db::MySqlDB;
use mysql::{DriverError::SetupError, Value};

impl VisualizeDB for MySqlDB {
    fn chrono_graph(&self, opts: ChronoAnalysisOpts) -> Result<AnalysisResults, DBError> {
        let ChronoAnalysisOpts {
            table,
            chrono_col,
            basis,
            range,
        } = opts;

        let start = range.start();
        let end = range.end();

        let xcol = "BASABLE_CHRONO_BASIS_VALUE";
        let ycol = "BASABLE_CHRONO_RESULT";

        let query = format!(
            "
            SELECT
                {basis}({chrono_col}) as {xcol},
                COUNT(*) as {ycol}
            FROM
                {table}
            WHERE
                {chrono_col} BETWEEN '{start}' AND '{end}'
            GROUP BY
                {basis}({chrono_col})
            ORDER BY
                BASABLE_CHRONO_BASIS_VALUE

        "
        );

        let conn = self.connector();
        let rows = conn.exec_query(&query)?;

        let results: AnalysisResults = rows
            .iter()
            .map(|r| {
                let x = match basis {
                    ChronoAnalysisBasis::Daily => {
                        let date: Date = r.get(xcol).unwrap();
                        AnalysisValue::Date(date)
                    }
                    _ => AnalysisValue::UInt(r.get(xcol).unwrap()),
                };

                let y = AnalysisValue::UInt(r.get(ycol).unwrap());

                AnalysisResult::new(x, y)
            })
            .collect();

        Ok(results)
    }

    fn trend_graph(&self, opts: TrendAnalysisOpts) -> Result<AnalysisResults, DBError> {
        let query = opts
            .build_query()
            .map_err(|_| mysql::Error::DriverError(SetupError));
        let query = query?;

        let TrendAnalysisOpts {
            xcol,
            ycol,
            analysis_type,
            ..
        } = opts;

        let conn = self.connector();
        let rows = conn.exec_query(&query)?;

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
