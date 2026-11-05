use calamine::Data;
use rust_xlsxwriter::*;

use crate::utils::data::{Heading, ColumnType};
use crate::utils::formats::{CellFormat, Formats};

pub struct Writer {
    workbook: Workbook,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            workbook: Workbook::new(),
        }
    }

    pub fn save<P: AsRef<std::path::Path>>(&mut self, file_name: P) -> Result<(), XlsxError> {
        self.workbook.save(file_name)
    }

    pub fn add_worksheet<'a>(&'a mut self, table_name: &str) -> Result<Sheet<'a>, XlsxError> {
        let worksheet = self.workbook.add_worksheet();        
        worksheet.set_name(table_name)?;
        Ok(Sheet::new(worksheet))
    }
}

pub struct Sheet<'a> {
    worksheet: &'a mut Worksheet,
}

impl <'a> Sheet<'a> {
     pub fn new(worksheet: &'a mut Worksheet) -> Self {
        Self { worksheet }
    }

    #[allow(unused)]
    pub fn write_header(
        &mut self,
        col: u16,
        text: &str,
        formats: &Formats,
    ) -> Result<(), XlsxError> {
        self.worksheet.write_with_format(
            0,
            col,
            text,
            formats.get(CellFormat::Header),
        )?;

        Ok(())
    }

    #[allow(unused)]
    pub fn write_column(
        &mut self,
        col: u16,
        values: &[Data],
        format: &Format,
    ) -> Result<(), XlsxError> {
        for (row, value) in values.iter().enumerate() {
            self.write_data(
                row as u32 + 1,
                col,
                value,
                format,
            )?;
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn write_data(
        &mut self,
        row: u32,
        col: u16,
        value: &Data,
        format: &Format,
    ) -> Result<(), XlsxError> {
        match value {
            Data::Int(v) => {
                self.worksheet.write_with_format(row, col, *v, format)?;
            }

            Data::Float(v) => {
                self.worksheet.write_with_format(row, col, *v, format)?;
            }

            Data::String(v) => {
                self.worksheet.write_with_format(row, col, v, format)?;
            }

            Data::Bool(v) => {
                self.worksheet.write_with_format(row, col, *v, format)?;
            }

            Data::DateTime(dt) => {
                self.worksheet
                    .write_with_format(row, col, dt.as_f64(), format)?;
            }

            _ => {
                self.worksheet.write_with_format(
                    row,
                    col,
                    value.to_string(),
                    format,
                )?;
            }
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn write_table(
        &mut self,
        table: &Heading,
        formats: &Formats,
    ) -> Result<(), XlsxError> {
        for (col_idx, column) in table.columns().iter().enumerate() {

            self.write_header(
                col_idx as u16,
                column.heading(),
                formats,
            )?;

            let fmt = match column.col_type() {
                ColumnType::Text =>
                    formats.get(CellFormat::Normal),

                ColumnType::Integer =>
                    formats.get(CellFormat::Integer),

                ColumnType::Number =>
                    formats.get(CellFormat::Number),

                ColumnType::Date =>
                    formats.get(CellFormat::Date),

                ColumnType::Euro =>
                    formats.get(CellFormat::Euro),
            };

            self.write_column(
                col_idx as u16,
                column.values(),
                fmt,
            )?;
        }

        Ok(())
    }

    pub fn set_column_width(&mut self, col: u16, width: u32) -> Result<&mut Worksheet, XlsxError> {
        self.worksheet.set_column_width(col, width)
    }
}