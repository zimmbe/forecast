use std::path::PathBuf;
//use std::error::Error;
//use std::fs;

use calamine::{Data, DataType};
use forecast::utils::data::*;
use forecast::utils::formats::*;
use forecast::utils::read::*;
use forecast::utils::time::*;
use forecast::utils::write::*;

fn test_file(name: &str) -> PathBuf {
    PathBuf::from("resources").join(name)
}

#[test]
fn test_roundtrip() {
    let file = test_file("test_write_read.xlsx");

    // create table
    let mut builder = HeadingBuilder::new(5);
    builder = builder
        .add("Nr", ColumnType::Integer)
        .add("Name", ColumnType::Text)
        .add("Datum", ColumnType::Date)
        .add("Stundenpreis", ColumnType::Euro)
        .add("Stunden", ColumnType::Number);
    let mut table = builder.build().unwrap();
    println!("headings: {:?}", table);
    let _ = table.add_column_values(0, vec![Data::Int(1), Data::Int(2), Data::Int(3)]);
    let _ = table.add_column_values(
        1,
        vec![
            Data::String("Maier".to_string()),
            Data::String("Müller".to_string()),
            Data::String("Huber".to_string()),
        ],
    );
    let _ = table.add_column_values(
        2,
        vec![
            make_date(2026, 10, 28, false).unwrap(),
            make_date(2026, 11, 5, false).unwrap(),
            make_date(2026, 11, 9, false).unwrap(),
        ],
    );
    let _ = table.add_column_values(
        3,
        vec![Data::Float(82.50), Data::Int(120), Data::Float(95.75)],
    );
    let _ = table.add_column_values(
        4,
        vec![Data::Float(8.5), Data::Float(7.0), Data::Float(9.75)],
    );
    println!("table: {:?}", table);

    // write
    let formats = Formats::new();
    let mut writer = Writer::new();
    let mut sheet = writer.add_worksheet("Test").unwrap();
    let res = sheet.write_table(&table, &formats);
    let _ = sheet.set_column_widths(0, 5, 15);
    assert!(res.is_ok(), "Sheet generation failed!");

    let xres = writer.save(&file);
    assert!(xres.is_ok(), "Saving file failed!");

    // read
    let mut reader = TableReader::open(&file).unwrap();
    let cfg = Config::new();
    let result = reader.read_table("Test", &cfg).unwrap();

    assert_eq!(result.get_column_headings()[0], "Nr");
    let _column = result.get_column("Nr").unwrap();

    assert_eq!(
        result.get_value("Nr", 0).unwrap().as_i64(),
        Data::Int(1).as_i64()
    );
    assert_eq!(
        result.get_value("Nr", 1).unwrap().as_i64(),
        Data::Int(2).as_i64()
    );
    assert_eq!(
        result.get_value("Nr", 2).unwrap().as_i64(),
        Data::Int(3).as_i64()
    );
}
