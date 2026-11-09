use calamine::{Data, DataType};
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
                        "ColumnType::Integer erwartet Integer, erhalten: {:?}",
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
                        "ColumnType::Number erwartet f64 oder i64, erhalten: {:?}",
                        value
                    )
                    .into());
                }
            }

            ColumnType::Text => {
                let v = value.as_string().ok_or_else(|| {
                    format!(
                        "ColumnType::Text erwartet String, erhalten: {:?}",
                        value
                    )
                })?;

                self.worksheet.write_with_format(row, col, v, format)?;
            }          

            ColumnType::Date => {
               /*if let Data::DateTime(dt) = value {
                    self.worksheet
                        .write_with_format(row, col, dt.as_f64(), format)?;
                }*/
                let dt = match value {
                    Data::DateTime(dt) => {
                        dt
                    }
                    _ => {
                        return Err(format!(
                            "ColumnType::Date erwartet Data::DateTime, erhalten: {:?}",
                            value
                        )
                        .into());
                    }
                };           

                self.worksheet.write_with_format(row, col, dt.as_f64(), format)?;
            }

            ColumnType::Time => {
               /*if let Data::DateTime(dt) = value {
                    self.worksheet
                        .write_with_format(row, col, dt.as_f64(), format)?;
                }*/
                let dt = match value {
                    Data::DateTime(dt) => dt,
                    _ => {
                        return Err(format!(
                            "ColumnType::Time erwartet Data::DateTime, erhalten: {:?}",
                            value
                        )
                        .into());
                    }
                };

                self.worksheet.write_with_format(row, col, dt.as_f64(), format)?;
            }

            ColumnType::DateTime => {
               /*if let Data::DateTime(dt) = value {
                    self.worksheet
                        .write_with_format(row, col, dt.as_f64(), format)?;
                }*/
                let dt = match value {
                    Data::DateTime(dt) => dt,
                    _ => {
                        return Err(format!(
                            "ColumnType::DateTime erwartet Data::DateTime, erhalten: {:?}",
                            value
                        )
                        .into());
                    }
                };

                self.worksheet.write_with_format(row, col, dt.as_f64(), format)?;
            }

            ColumnType::Duration => {
               dbg!(col_type);
                let dt = match value {
                    Data::DateTime(dt) => dt,
                    _ => {
                        return Err(format!(
                            "ColumnType::Duration erwartet Data::DateTime, erhalten: {:?}",
                            value
                        )
                        .into());
                    }
                };

                let duration = dt.as_duration().ok_or_else(|| {
                    format!(
                        "ColumnType::Duration erwartet eine Duration, erhalten: {:?}",
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
                        "ColumnType::Euro erwartet Data::Floa oder Data::Int, erhalten: {:?}",
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
                            "ColumnType::Percentage erwartet Data::Float, erhalten: {:?}",
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
                            "ColumnType::Bool erwartet Data::Bool, erhalten: {:?}",
                            value
                        )
                        .into());
                    }
                };

                self.worksheet.write_with_format(row, col, v, format)?;
            }

            // TODO: not yet handled the following cases, because not expected to be 
            // Data::Data:DateTimeIso, Data:DurationIso

            _ => {
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
    pub fn write_table(&mut self, table: &Heading, formats: &Formats) -> Result<(), Box<dyn std::error::Error>> {
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

    /*pub fn match_column_tpye_with_value(&mut self, col_type: ColumnType, value: Data) -> Result<(), Error> {
        let cell_type = match col_type {
            ColumnType::Text => matches(value.is_string()),
            ColumnType::Integer => value.is_int(),
            ColumnType::Number => value.is_float(),
            ColumnType::Date => value.is_datetime(),
            ColumnType::Time => value.is_datetime(),
            ColumnType::DateTime => value.is_datetime(),
            ColumnType::Duration => value.is_datetime(),
            ColumnType::Euro => value.is_int(),
        }
    }*/
}
