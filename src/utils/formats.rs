use crate::utils::data::ColumnType;
use rust_xlsxwriter::Format;

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum CellFormat {
    Header,
    Normal,
    Bold,
    Number,
    Integer,
    Date,
    Time,
    DateTime,
    Duration,
    Euro,
    Percentage,
    Bool,
}

pub struct Formats {
    pub header: Format,
    pub normal: Format,
    pub bold: Format,
    pub number: Format,
    pub integer: Format,
    pub date: Format,
    pub time: Format,
    pub datetime: Format,
    pub duration: Format,
    pub euro: Format,
    pub percentage: Format,
    pub bool: Format,
}

impl Formats {
    pub fn new() -> Self {
        let font_name = "Calibri";

        Self {
            header: Format::new()
                .set_font_name(font_name)
                .set_font_size(12)
                .set_bold(),

            normal: Format::new().set_font_name(font_name).set_font_size(10),
            bold: Format::new()
                .set_font_name(font_name)
                .set_font_size(10)
                .set_bold(),
            number: Format::new()
                .set_font_name(font_name)
                .set_font_size(10)
                .set_num_format("0.00"),
            integer: Format::new()
                .set_font_name(font_name)
                .set_font_size(10)
                .set_num_format("0"),
            date: Format::new()
                .set_font_name(font_name)
                .set_font_size(10)
                .set_num_format("dd.mm.yyyy"),
            time: Format::new()
                .set_font_name(font_name)
                .set_font_size(10)
                .set_num_format("hh:mm:ss"),
            datetime: Format::new()
                .set_font_name(font_name)
                .set_font_size(10)
                .set_num_format("dd.mm.yyyy hh:mm:ss"),
            duration: Format::new()
                .set_font_name(font_name)
                .set_font_size(10)
                .set_num_format("[h]:mm:ss"),
            euro: Format::new()
                .set_font_name(font_name)
                .set_font_size(10)
                .set_num_format("#,##0.00 €"),
            percentage: Format::new()
                .set_font_name(font_name)
                .set_font_size(10)
                .set_num_format("0.00 %"),
            bool: Format::new().set_font_name(font_name).set_font_size(10),
        }
    }

    pub fn get(&self, fmt: CellFormat) -> &Format {
        match fmt {
            CellFormat::Header => &self.header,
            CellFormat::Normal => &self.normal,
            CellFormat::Bold => &self.bold,
            CellFormat::Number => &self.number,
            CellFormat::Integer => &self.integer,
            CellFormat::Date => &self.date,
            CellFormat::Time => &self.time,
            CellFormat::DateTime => &self.datetime,
            CellFormat::Duration => &self.duration,
            CellFormat::Euro => &self.euro,
            CellFormat::Percentage => &self.percentage,
            CellFormat::Bool => &self.bool,
        }
    }
}

impl From<&ColumnType> for CellFormat {
    fn from(col_type: &ColumnType) -> Self {
        dbg!("from", col_type);
        match col_type {
            ColumnType::Text => CellFormat::Normal,
            ColumnType::Integer => CellFormat::Integer,
            ColumnType::Number => CellFormat::Number,
            ColumnType::Date => CellFormat::Date,
            ColumnType::Time => CellFormat::Time,
            ColumnType::DateTime => CellFormat::DateTime,
            ColumnType::Duration => CellFormat::Duration,
            ColumnType::Euro => CellFormat::Euro,
            ColumnType::Percentage => CellFormat::Percentage,
            ColumnType::Bool => CellFormat::Bool,
        }
    }
}
