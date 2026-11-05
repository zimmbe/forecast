use calamine::{Data, Range, Reader, Xlsx, XlsxError, open_workbook};
use std::path::Path;

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
    pub fn get_sheet_name(&mut self, n: usize) -> String {
        self.workbook.sheet_names()[n].to_string()
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

            if let Data::String(s) = cell {
                if s.eq_ignore_ascii_case("Heading6") {
                    col_type = ColumnType::Euro;
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
            Data::Int(_) => ColumnType::Number,
            Data::Float(_) => ColumnType::Number,
            Data::DateTime(_) => ColumnType::Date,
            Data::DateTimeIso(_) => ColumnType::Date,
            _ => ColumnType::Text,
        }
    }
}
