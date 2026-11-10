use calamine::{Data, DataType};
use log::debug;
use rust_xlsxwriter::*;

use crate::utils::data::{ColumnType, Heading};
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

impl<'a> Sheet<'a> {
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
        self.worksheet
            .write_with_format(0, col, text, formats.get(CellFormat::Header))?;

        Ok(())
    }

    #[allow(unused)]
    pub fn write_cell(
        &mut self,
        row: u32,
        col: u16,
        value: &Data,
        col_type: &ColumnType,
        format: &Format,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match col_type {
            ColumnType::Integer => {
                let v = value.as_i64().ok_or_else(|| {
                    format!(
                        "ColumnType::Integer expects Data::Int, received: {:?}",
                        value
                    )
                })?;

                self.worksheet.write_with_format(row, col, v, format)?;
            }

            ColumnType::Number => {
                if let Some(v) = value.as_f64() {
                    self.worksheet.write_with_format(row, col, v, format)?;
                } else if let Some(v) = value.as_i64() {
                    self.worksheet.write_with_format(row, col, v, format)?;
                } else {
                    return Err(format!(
                        "ColumnType::Number expected f64 oder i64, received: {:?}",
                        value
                    )
                    .into());
                }
            }

            ColumnType::Text => {
                let v = value.as_string().ok_or_else(|| {
                    format!("ColumnType::Text expected String, received: {:?}", value)
                })?;

                self.worksheet.write_with_format(row, col, v, format)?;
            }

            ColumnType::Date => {
                let dt = match value {
                    Data::DateTime(dt) => dt,
                    _ => {
                        return Err(format!(
                            "ColumnType::Date expected Data::DateTime, received: {:?}",
                            value
                        )
                        .into());
                    }
                };

                self.worksheet
                    .write_with_format(row, col, dt.as_f64(), format)?;
            }

            ColumnType::Time => {
                let dt = match value {
                    Data::DateTime(dt) => dt,
                    _ => {
                        return Err(format!(
                            "ColumnType::Time expected Data::DateTime, received: {:?}",
                            value
                        )
                        .into());
                    }
                };

                self.worksheet
                    .write_with_format(row, col, dt.as_f64(), format)?;
            }

            ColumnType::DateTime => {
                let dt = match value {
                    Data::DateTime(dt) => dt,
                    _ => {
                        return Err(format!(
                            "ColumnType::DateTime expected Data::DateTime, received: {:?}",
                            value
                        )
                        .into());
                    }
                };

                self.worksheet
                    .write_with_format(row, col, dt.as_f64(), format)?;
            }

            ColumnType::Duration => {
                dbg!(col_type);
                let dt = match value {
                    Data::DateTime(dt) => dt,
                    _ => {
                        return Err(format!(
                            "ColumnType::Duration expected Data::DateTime, received: {:?}",
                            value
                        )
                        .into());
                    }
                };

                let duration = dt.as_duration().ok_or_else(|| {
                    format!(
                        "ColumnType::Duration expected eine Duration, received: {:?}",
                        dt
                    )
                })?;

                dbg!(duration);

                self.worksheet
                    .write_with_format(row, col, dt.as_f64(), format)?;
            }

            ColumnType::Euro => {
                if let Some(v) = value.as_f64() {
                    self.worksheet.write_with_format(row, col, v, format)?;
                } else if let Some(v) = value.as_i64() {
                    self.worksheet.write_with_format(row, col, v, format)?;
                } else {
                    return Err(format!(
                        "ColumnType::Euro expected Data::Floa oder Data::Int, received: {:?}",
                        value
                    )
                    .into());
                }
            }

            ColumnType::Percentage => {
                let v = match value {
                    Data::Float(v) => *v,
                    _ => {
                        return Err(format!(
                            "ColumnType::Percentage expected Data::Float, received: {:?}",
                            value
                        )
                        .into());
                    }
                };
                self.worksheet.write_with_format(row, col, v, format)?;
            }

            ColumnType::Bool => {
                let v = match value {
                    Data::Bool(v) => *v,
                    _ => {
                        return Err(format!(
                            "ColumnType::Bool expected Data::Bool, received: {:?}",
                            value
                        )
                        .into());
                    }
                };

                self.worksheet.write_with_format(row, col, v, format)?;
            }

            // TODO: not yet handled the following cases, because not expected to be availble
            // Data::Data:DateTimeIso, Data:DurationIso
            // these types do not occur in real excel file, comes from Libre Office, ODS etc.
            _ => {
                debug!("Unhandled column type {:?}, write as Text", col_type);
                self.worksheet
                    .write_with_format(row, col, value.to_string(), format)?;
            }
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn write_column(
        &mut self,
        col: u16,
        values: &[Data],
        col_type: &ColumnType,
        format: &Format,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (row, value) in values.iter().enumerate() {
            self.write_cell(row as u32 + 1, col, value, col_type, format)?;
        }
        Ok(())
    }

    #[allow(unused)]
    pub fn write_table(
        &mut self,
        table: &Heading,
        formats: &Formats,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (col_idx, column) in table.columns().iter().enumerate() {
            self.write_header(col_idx as u16, column.heading(), formats)?;

            let fmt = match column.col_type() {
                ColumnType::Text => formats.get(CellFormat::Normal),
                ColumnType::Integer => formats.get(CellFormat::Integer),
                ColumnType::Number => formats.get(CellFormat::Number),
                ColumnType::Date => formats.get(CellFormat::Date),
                ColumnType::Time => formats.get(CellFormat::Time),
                ColumnType::DateTime => formats.get(CellFormat::DateTime),
                ColumnType::Duration => formats.get(CellFormat::Duration),
                ColumnType::Euro => formats.get(CellFormat::Euro),
                ColumnType::Percentage => formats.get(CellFormat::Percentage),
                ColumnType::Bool => formats.get(CellFormat::Bool),
            };

            self.write_column(col_idx as u16, column.values(), column.col_type(), fmt)?;
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn set_column_width(&mut self, col: u16, width: u32) -> Result<&mut Worksheet, XlsxError> {
        self.worksheet.set_column_width(col, width)
    }

    #[allow(unused)]
    pub fn set_column_widths(&mut self, col_from: u16, col_to: u16, width: u32) -> Result<(), XlsxError> {
        for col in col_from..=col_to {
            self.worksheet.set_column_width(col, width)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::data;
    use calamine::Data;

    fn create_test_environment() -> (Writer, Formats) {
        (Writer::new(), Formats::new())
    }

    fn create_test_sheet<'a>(
        col_type: ColumnType,
        cell_format: CellFormat,
        value: &Data,
    ) -> Result<(), Box<dyn std::error::Error>> /*(Sheet<'a>, &Format)*/ {
        let (mut writer, fmts) = create_test_environment();
        let mut sheet = writer.add_worksheet("Tab1").unwrap();
        let fmt = fmts.get(cell_format);
        sheet.write_cell(
            0, 0, value, //&Data::Float(1.0),
            &col_type, fmt,
        )
    }

    #[test]
    fn test_number_accepts_int() {
        let res = create_test_sheet(ColumnType::Integer, CellFormat::Integer, &Data::Int(42));

        assert!(res.is_ok());
    }

    #[test]
    fn test_euro_accepts_int() {
        let res = create_test_sheet(ColumnType::Euro, CellFormat::Euro, &Data::Int(42));

        assert!(res.is_ok());
    }

    #[test]
    fn test_euro_accepts_float() {
        let res = create_test_sheet(ColumnType::Euro, CellFormat::Euro, &Data::Float(5.87));

        assert!(res.is_ok());
    }

    #[test]
    fn test_percentage_accepts_float() {
        let res = create_test_sheet(
            ColumnType::Percentage,
            CellFormat::Percentage,
            &Data::Float(0.3333),
        );

        assert!(res.is_ok());
    }

    #[test]
    fn test_percentage_rejects_int() {
        let res = create_test_sheet(
            ColumnType::Percentage,
            CellFormat::Percentage,
            &Data::Int(2),
        );

        assert!(res.is_err());
    }

    #[test]
    fn test_write_integer_with_string() {
        let err = create_test_sheet(
            ColumnType::Integer,
            CellFormat::Integer,
            &Data::String("abc".into()),
        )
        .unwrap_err();

        assert!(err.to_string().contains("ColumnType::Integer"));
    }

    #[test]
    fn test_write_bool_with_float() {
        let err =
            create_test_sheet(ColumnType::Bool, CellFormat::Bool, &Data::Float(1.0)).unwrap_err();

        assert!(err.to_string().contains("ColumnType::Bool"));
    }

    #[test]
    fn test_write_date_with_string() {
        let err = create_test_sheet(
            ColumnType::Date,
            CellFormat::Date,
            &Data::String("2026-01-01".into()),
        )
        .unwrap_err();

        assert!(err.to_string().contains("ColumnType::Date"));
    }

    #[test]
    fn test_write_column_propagates_error() {
        let values = vec![Data::Int(1), Data::String("oops".into())];
        let (mut writer, fmts) = create_test_environment();
        let mut sheet = writer.add_worksheet("Tab1").unwrap();
        let fmt = fmts.get(CellFormat::Integer);
        let err = sheet
            .write_column(0, &values, &ColumnType::Integer, &fmt)
            .unwrap_err();

        assert!(err.to_string().contains("ColumnType") && err.to_string().contains("expects"));
    }

    #[test]
    fn test_write_table_propagates_error() {
        let mut builder = data::HeadingBuilder::new(1);

        builder = builder.add("Nr", ColumnType::Integer);

        let mut table = builder.build().unwrap();

        let err = table
            .column_mut(0)
            .unwrap()
            .push(Data::String("abc".into()))
            .unwrap_err();

        assert!(err.to_string().contains("Invalid data type") && err.to_string().contains("abc"));
    }
}
