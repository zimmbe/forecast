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

fn write_and_read_table() -> Table {
    let file = test_file("test_write_read.xlsx");

    // create table
    let mut builder = TableBuilder::new(5);
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

    // write new excel file
    let formats = Formats::new();
    let mut writer = Writer::new();
    let mut sheet = writer.add_worksheet("Test").unwrap();
    let _res = sheet.write_table(&table, &formats);
    let _ = sheet.set_column_widths(0, 5, 15);
    let _xres = writer.save(&file);

    // read in the previously created excel file
    let mut reader = TableReader::open(&file).unwrap();
    let cfg = Config::new();

    reader.read_table("Test", &cfg).unwrap()
}

#[test]
fn test_roundtrip_integer() {
    let result = write_and_read_table();
    let col_name = "Nr";
    assert_eq!(
        result.get_value(col_name, 0).unwrap().as_i64(),
        Data::Int(1).as_i64()
    );
    assert_eq!(
        result.get_value(col_name, 1).unwrap().as_i64(),
        Data::Int(2).as_i64()
    );
    assert_eq!(
        result.get_value(col_name, 2).unwrap().as_i64(),
        Data::Int(3).as_i64()
    );
}

#[test]
fn test_roundtrip_number() {
    let result = write_and_read_table();
    let col_name = "Stunden";
    assert_eq!(
        result.get_value(col_name, 0).unwrap().as_f64(),
        Data::Float(8.5).as_f64()
    );
    assert_eq!(
        result.get_value(col_name, 1).unwrap().as_f64(),
        Data::Float(7.0).as_f64()
    );
    assert_eq!(
        result.get_value(col_name, 2).unwrap().as_f64(),
        Data::Float(9.75).as_f64()
    );
}

#[test]
fn test_roundtrip_text() {
    let result = write_and_read_table();
    let col_name = "Name";
    assert_eq!(
        result.get_value(col_name, 0).unwrap().as_string(),
        Data::String("Maier".into()).as_string()
    );
    assert_eq!(
        result.get_value(col_name, 1).unwrap().as_string(),
         Data::String("Müller".into()).as_string()
    );
    assert_eq!(
        result.get_value(col_name, 2).unwrap().as_string(),
         Data::String("Huber".into()).as_string()
    );
}

#[test]
fn test_roundtrip_currency() {
    let result = write_and_read_table();
    let col_name = "Stundenpreis";
    assert_eq!(
        result.get_value(col_name, 0).unwrap().as_f64(),
        Data::Float(82.5).as_f64()
    );
    assert_eq!(
        result.get_value(col_name, 1).unwrap().as_f64(),
        Data::Float(120.0).as_f64()
    );
    assert_eq!(
        result.get_value(col_name, 2).unwrap().as_f64(),
        Data::Float(95.75).as_f64()
    );
}

#[test]
fn test_roundtrip_datetime() {
    let result = write_and_read_table();
    let col_name = "Datum";
    let times = vec![make_date(2026, 10, 28, false),
        make_date(2026, 11, 5, false),
        make_date(2026, 11, 9, false)
    ];

    assert_eq!(
        datetime_value(result.get_value(col_name, 0).unwrap()),
        datetime_value(times[0].as_ref().unwrap())
    );

    assert_eq!(
        datetime_value(result.get_value(col_name, 1).unwrap()),
        datetime_value(times[1].as_ref().unwrap())
    );
    assert_eq!(
        datetime_value(result.get_value(col_name, 2).unwrap()),
        datetime_value(times[2].as_ref().unwrap())
    );
}
