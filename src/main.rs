mod utils;

use log::{debug, error, info};
//use rust_xlsxwriter::ExcelDateTime;
use calamine::{Data, DataType};
use std::path::Path;
use utils::data_xlsx as data;
use utils::formats;
use utils::read;
use utils::write;

use crate::utils::data_xlsx::ColumnType;
use crate::utils::data_xlsx::DataExt;
 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();
    info!("start forecast");

    let file_in: &Path = Path::new("test.xlsx");
    let file_out: &Path = Path::new("out.xlsx");
    let table = "Forecast";
    debug!("file: {:?}", file_in);
    let formats = formats::Formats::new();

    // define a new data structure with own headings
    let num_cols: usize = 7;
    let mut new_table = data::HeadingBuilder::new(num_cols)
        .add("Nr", ColumnType::Integer)
        .add("Name", ColumnType::Text)
        .add("Stunden", ColumnType::Number)
        .add("Faktor", ColumnType::Number)
        .add("Datum", ColumnType::Date)
        .add("Stundenpreis", ColumnType::Euro)
        .add("Gesamtpreis", ColumnType::Euro)
        .build()?;
    info!("Headings:\n{new_table:?}");

    // read excel
    let mut reader = read::TableReader::open(file_in)
        .inspect_err(|err| error!("read {:?}: {err:?}", file_in.as_os_str()))?;

    let sheets = reader.get_sheets();
    info!("sheets in workbook: {:?}", sheets);
    let sheet_name = reader.get_sheet_name(1);
    let (_r, _c) = reader.get_rows_and_cols(&sheet_name);

    let read_table = reader.read_table(table)?;
    info!("read_table direct: {:?}", read_table);
    info!("read_table formatted:");
    let mut idx = 0;
    for column in read_table.columns() {
        println!("----- {}/{:?} -----", column.heading(), column.col_type());

        for value in column.values() {
            print!("{}; ", value.as_display_string());
        }
        print!("\n");
        new_table.add_column_values(idx, column.values().to_vec())?;
        idx += 1;
    }
    // do some calculations
    let h2 = new_table.get_column_heading(2);
    let h3 = new_table.get_column_heading(3);
    let h5 = new_table.get_column_heading(5);
    let vec_hours = new_table.get_column(&h2).expect("Spalte fehlt");
    let vec_factor = new_table.get_column(&h3).expect("Spalte fehlt");
    let vec_price_single = new_table.get_column(&h5).expect("Spalte fehlt");

    let values: Vec<Data> = vec_hours
        .values()
        .iter()
        .zip(vec_factor.values().iter())
        .zip(vec_price_single.values().iter())
        .map(|((hours, factor), price)| {
            let hours = hours.as_f64().unwrap_or(0.0);
            let factor = factor.as_f64().unwrap_or(0.0);
            let price = price.as_f64().unwrap_or(0.0);

            Data::Float(hours * factor * price)
        })
        .collect();
    // add calculation results to the new table
    new_table.add_column_values(6, values)?;
    info!("new_table: {:?}", new_table);

    // write new excel file
    let mut writer = write::Writer::new();
    {
        let mut sheet = writer.add_worksheet(table)?;
        sheet.write_table(&new_table, &formats)?;
        sheet.set_column_width(0, 10)?;
        for i in 1..num_cols as u16 {
            sheet.set_column_width(i, 15)?;
    }
    }
    debug!("file: {:?}", file_out);
    writer.save(file_out)?;

    Ok(())
}
