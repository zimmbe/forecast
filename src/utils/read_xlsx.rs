use calamine::{Data, Range, Reader, Xlsx, XlsxError, open_workbook};
use std::path::Path;
use log::error;

use crate::utils::data::{ColumnType, Heading, HeadingBuilder};

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
        .ok_or_else(|| {
            format!("Tabellenblatt mit Index {n} existiert nicht")
                .into()
        })
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

    pub fn read_table(&mut self, table_name: &str) -> Result<Heading, Box<dyn std::error::Error>> {
        let range = self.workbook.worksheet_range(table_name)?;

        let mut rows = range.rows();

        let header_row = rows.next().ok_or("Keine Kopfzeile vorhanden")?;

        let first_row = rows.next().clone().ok_or("Keine Datenzeile vorhanden")?;

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
                if s.to_lowercase().contains("preis") ||  s.contains("€") {
                    col_type = ColumnType::Euro;
                }
                if s.to_lowercase().contains("perc") ||  s.contains("%") {
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
                println!("dt full: {:?}",dt.to_ymd_hms_milli());
                let value: f64 = (23.0+12.0/60.0)/24.0;
                //let dur = ExcelDateTime::new(value, TimeDelta, false);
                if dt.is_duration() {
                    dbg!("set Duration");
                    ColumnType::Duration
                } else if let Some(datetime) = dt.as_datetime() {
                    if datetime.time() == chrono::NaiveTime::default() {                        
                        dbg!("set Date");
                        ColumnType::Date                    
                    } else if dt.to_ymd_hms_milli().0 < 1900 {
                        if dt.as_f64() == value {
                            ColumnType::Duration
                        } else {
                        dbg!("set Time");
                        ColumnType::Time
                        }
                    } else {
                        dbg!("set DateTime");
                        ColumnType::DateTime   
                    }
                } else {
                    error!("Ungültiges Datumsformat: weder datetime noch duration => setze ColumnType::Text");
                    ColumnType::Text
                }
            }
            // these types do not occur in real excel file, comes from Libre Office, ODS etc.
            // xml must contain <c t="d">      <!-- ISO Date -->
            Data::DateTimeIso(_) => ColumnType::DateTime,  // e.g. 2026-11-06T08:30:00+01:00
            Data::DurationIso(_) => ColumnType::Duration,  // e.g. PT2H30M45S = 2h 30min 45s
            _ => ColumnType::Text,
        }
    }
}
