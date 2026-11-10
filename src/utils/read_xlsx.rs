use calamine::{Data, Range, Reader, Xlsx, XlsxError, open_workbook};
use log::{debug, error};
use std::path::Path;

use crate::utils::data::{ColumnType, Heading, HeadingBuilder};

#[derive(Debug, Clone)]
pub struct Config {
    required_columns: Vec<String>,
    currency_strings: Vec<String>,
    percentage_strings: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            required_columns: Vec::new(),
            currency_strings: Vec::new(),
            percentage_strings: Vec::new(),
        }
    }

    pub fn add_required_column(&mut self, name: &str) {
        self.required_columns.push(name.into());
    }

    pub fn add_currency_string(&mut self, name: &str) {
        self.currency_strings.push(name.into());
    }

    pub fn add_percentage_string(&mut self, name: &str) {
        self.percentage_strings.push(name.into());
    }

    pub fn required_columns(&self) -> &[String] {
        &self.required_columns
    }
}

pub struct TableReader {
    workbook: Xlsx<std::io::BufReader<std::fs::File>>,
}

impl TableReader {
    pub fn open<P: AsRef<Path>>(file_name: P) -> Result<Self, XlsxError> {
        Ok(Self {
            workbook: open_workbook(file_name)?,
        })
    }

    #[allow(unused)]
    pub fn get_sheets(&mut self) -> Vec<String> {
        self.workbook.sheet_names()
    }

    #[allow(unused)]
    pub fn get_sheet_name(&mut self, n: usize) -> Result<String, Box<dyn std::error::Error>> {
        self.workbook
            .sheet_names()
            .get(n)
            .map(|s| s.to_string())
            .ok_or_else(|| format!("Tabellenblatt mit Index {n} existiert nicht").into())
    }

    #[allow(unused)]
    pub fn get_rows_and_cols(&mut self, table: &str) -> (usize, usize) {
        if let Ok(range) = self.workbook.worksheet_range(table) {
            range.get_size()
        } else {
            (0, 0)
        }
    }

    #[allow(unused)]
    pub fn open_table<P: AsRef<Path>>(file_name: P, table: &str) -> Result<Range<Data>, XlsxError> {
        let mut workbook: Xlsx<_> = open_workbook(file_name)?;

        workbook
            .worksheet_range(table)
            .map(|sheet| sheet.to_owned())
    }

    pub fn read_table(
        &mut self,
        table_name: &str,
        cfg: &Config,
    ) -> Result<Heading, Box<dyn std::error::Error>> {
        let range = self.workbook.worksheet_range(table_name)?;

        let mut rows = range.rows();

        let header_row = rows.next().ok_or("No header present")?;

        let first_row = rows.next().clone().ok_or("No data row available")?;

        let mut builder = HeadingBuilder::new(header_row.len());

        for (idx, cell) in header_row.iter().enumerate() {
            let heading = match cell {
                Data::String(s) => s.clone(),
                _ => cell.to_string(),
            };

            let mut col_type = first_row
                .get(idx)
                .map(Self::get_column_type)
                .unwrap_or(ColumnType::Text);

            // TODO: definiere Spaltenüberschriften, die Währung/Euro oder Prozentwerte enthalten
            if let Data::String(s) = cell {
                if s.eq_ignore_ascii_case("Heading6") {
                    col_type = ColumnType::Euro;
                }
                if cfg.currency_strings.contains(&s) {
                    col_type = ColumnType::Euro;
                }
                if cfg.percentage_strings.contains(&s) {
                    col_type = ColumnType::Percentage;
                }
            }

            builder = builder.add(heading, col_type);
        }

        let mut table = builder.build()?;

        // verarbeite erste Datenzeile
        for (col_idx, cell) in first_row.iter().enumerate() {
            if let Some(column) = table.column_mut(col_idx) {
                column.push(cell.clone())?;
            }
        }
        // alle weiteren Datenzeilen verarbeiten
        for row in rows {
            for (col_idx, cell) in row.iter().enumerate() {
                if let Some(column) = table.column_mut(col_idx) {
                    column.push(cell.to_owned())?;
                }
            }
        }

        Ok(table)
    }

    fn get_column_type(cell: &Data) -> ColumnType {
        match cell {
            Data::String(_) => ColumnType::Text,
            Data::Int(_) => ColumnType::Integer,
            Data::Float(v) if (v.fract().abs()) < f64::EPSILON => ColumnType::Integer,
            Data::Float(_) => ColumnType::Number,
            Data::Bool(_) => ColumnType::Bool,
            Data::DateTime(dt) => {
                debug!("dt full: {:?}", dt.to_ymd_hms_milli());
                let value: f64 = (23.0 + 12.0 / 60.0) / 24.0;

                if dt.is_duration() {
                    debug!("set Duration");
                    ColumnType::Duration
                } else if let Some(datetime) = dt.as_datetime() {
                    if datetime.time() == chrono::NaiveTime::default() {
                        debug!("set Date");
                        ColumnType::Date
                    } else if dt.to_ymd_hms_milli().0 < 1900 {
                        if dt.as_f64() == value {
                            ColumnType::Duration
                        } else {
                            debug!("set Time");
                            ColumnType::Time
                        }
                    } else {
                        debug!("set DateTime");
                        ColumnType::DateTime
                    }
                } else {
                    error!(
                        "Invalid date format: neither datetime nor duration => setting ColumnType::Text"
                    );
                    ColumnType::Text
                }
            }
            // these types do not occur in real excel file, comes from Libre Office, ODS etc.
            // xml must contain <c t="d">      <!-- ISO Date -->
            Data::DateTimeIso(_) => ColumnType::DateTime, // e.g. 2026-11-06T08:30:00+01:00
            Data::DurationIso(_) => ColumnType::Duration, // e.g. PT2H30M45S = 2h 30min 45s
            _ => ColumnType::Text,
        }
    }

    pub fn validate_columns(
        &mut self,
        table: &str,
        config: &Config,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let range = self.workbook.worksheet_range(table)?;

        let header_row = range.rows().next().ok_or("Table contains no headings")?;

        let headers: Vec<String> = header_row.iter().map(|c| c.to_string()).collect();

        for required in config.required_columns() {
            if !headers.iter().any(|h| h == required) {
                return Err(format!("Mandatory column '{}' is missing", required).into());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use calamine::{Data, ExcelDateTime, ExcelDateTimeType};

    #[test]
    fn test_get_column_type_integer() {
        assert_eq!(
            TableReader::get_column_type(&Data::Int(42)),
            ColumnType::Integer
        );
    }

    #[test]
    fn test_get_column_type_float_as_integer() {
        assert_eq!(
            TableReader::get_column_type(&Data::Float(42.0)),
            ColumnType::Integer
        );
    }

    #[test]
    fn test_get_column_type_number() {
        assert_eq!(
            TableReader::get_column_type(&Data::Float(24.58)),
            ColumnType::Number
        );
    }

    #[test]
    fn test_get_column_type_text() {
        assert_eq!(
            TableReader::get_column_type(&Data::String("Test".into())),
            ColumnType::Text
        );
    }

    #[test]
    fn test_get_column_type_bool() {
        assert_eq!(
            TableReader::get_column_type(&Data::Bool(true)),
            ColumnType::Bool
        );
    }

    #[test]
    fn test_get_column_type_date() {
        let date = ExcelDateTime::new(46330.0, ExcelDateTimeType::DateTime, false);
        println!("{:?}", date.to_ymd_hms_milli());
        assert_eq!(
            TableReader::get_column_type(&Data::DateTime(date)),
            ColumnType::Date
        );
    }

    #[test]
    fn test_get_column_type_time() {
        let time = ExcelDateTime::new(0.525, ExcelDateTimeType::DateTime, false);
        println!("{:?}", time.to_ymd_hms_milli());
        assert_eq!(
            TableReader::get_column_type(&Data::DateTime(time)),
            ColumnType::Time
        );
    }

    #[test]
    fn test_get_column_type_datetime() {
        let datetime = ExcelDateTime::new(46330.25125, ExcelDateTimeType::DateTime, false);
        println!("{:?}", datetime.as_datetime());
        assert_eq!(
            TableReader::get_column_type(&Data::DateTime(datetime)),
            ColumnType::DateTime
        );
    }

    #[test]
    fn test_get_column_type_duration() {
        let duration = ExcelDateTime::new(1.0 / 24.0, ExcelDateTimeType::TimeDelta, false);
        println!("{:?}", duration.as_duration());
        assert_eq!(
            TableReader::get_column_type(&Data::DateTime(duration)),
            ColumnType::Duration
        );
    }

    #[test]
    fn test_add_required_column() {
        let mut cfg = Config::new();

        cfg.add_required_column("Nr");
        cfg.add_required_column("Name");

        assert_eq!(
            cfg.required_columns(),
            &["Nr".to_string(), "Name".to_string()]
        );
    }
}
