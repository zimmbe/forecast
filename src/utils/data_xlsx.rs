use calamine::Data;

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum ColumnType {
    Text,
    Integer, // int
    Number,  // int or float
    Date,
    Time,
    DateTime,
    Duration,
    Euro,       // int or float
    Percentage, // enter as float, 0.25 = 25.00 %
    Bool,
}

impl ColumnType {
    // mapping of own ColumnType to datatype from calamine
    pub fn matches(&self, value: &Data) -> bool {
        match self {
            ColumnType::Text => matches!(value, Data::String(_)),
            ColumnType::Integer => matches!(value, Data::Int(_)),
            ColumnType::Number => matches!(value, Data::Float(_) | Data::Int(_)),
            ColumnType::Date => matches!(value, Data::DateTime(_)),
            ColumnType::Time => matches!(value, Data::DateTime(_)),
            ColumnType::DateTime => matches!(value, Data::DateTime(_)),
            ColumnType::Duration => matches!(value, Data::DateTime(_)),
            ColumnType::Euro => matches!(value, Data::Float(_) | Data::Int(_)),
            ColumnType::Percentage => matches!(value, Data::Float(_)),
            ColumnType::Bool => matches!(value, Data::Bool(_)),
        }
    }
}

pub trait DataExt {
    fn as_display_string(&self) -> String;
}

impl DataExt for Data {
    fn as_display_string(&self) -> String {
        match self {
            Data::DateTime(dt) => {
                if let Some(date) = dt.as_datetime() {
                    if date.time() == chrono::NaiveTime::default() {
                        date.format("%d.%m.%Y").to_string()
                    } else if dt.to_ymd_hms_milli().0 < 1900 {
                        date.format("%H:%M:%S").to_string()
                    } else {
                        date.format("%d.%m.%Y %H:%M:%S").to_string()
                    }
                } else if let Some(date) = dt.as_duration() {
                    date.to_string()
                } else {
                    dt.to_string()
                }
            }
            _ => self.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Column {
    heading: String,
    column_type: ColumnType,
    values: Vec<Data>,
}

impl Column {
    pub fn heading(&self) -> &str {
        &self.heading
    }

    pub fn col_type(&self) -> &ColumnType {
        &self.column_type
    }

    pub fn values(&self) -> &[Data] {
        &self.values
    }

    pub fn push(&mut self, value: Data) -> Result<(), String> {
        let valid = match (&self.column_type, &value) {
            // for Integer-Columnn Floats without decimal place are allowed
            (ColumnType::Integer, Data::Float(v)) => (v.fract().abs()) < f64::EPSILON,

            // default column type check for all other cases
            _ => self.column_type.matches(&value),
        };

        if !valid {
            return Err(format!(
                "Invalid data type in column '{}', '{:?}', val: {:?}",
                self.heading,
                self.col_type(),
                value
            ));
        }

        self.values.push(value);
        Ok(())
    }

    #[allow(unused)]
    pub fn values_mut(&mut self) -> &mut Vec<Data> {
        &mut self.values
    }
}

#[derive(Debug, PartialEq)]
pub struct Heading {
    columns: Vec<Column>,
}

impl Heading {
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    pub fn column_mut(&mut self, index: usize) -> Option<&mut Column> {
        self.columns.get_mut(index)
    }

    #[allow(unused)]
    pub fn add_column<S: Into<String>>(
        &mut self,
        heading: S,
        column_type: ColumnType,
    ) -> Result<(), String> {
        self.add_column_at(heading, column_type, self.columns.len())
    }

    #[allow(unused)]
    pub fn add_column_at<S: Into<String>>(
        &mut self,
        heading: S,
        column_type: ColumnType,
        n: usize,
    ) -> Result<(), String> {
        let heading = heading.into();

        if self.columns.iter().any(|c| c.heading == heading) {
            return Err(format!("Column '{}' already exists", heading));
        }

        if n > self.columns.len() {
            return Err(format!(
                "Invalid position {n}, maximum position is {}",
                self.columns.len()
            ));
        }

        let row_count = self.columns.first().map(|c| c.values.len()).unwrap_or(0);

        self.columns.insert(
            n,
            Column {
                heading,
                column_type,
                values: vec![Data::Empty; row_count],
            },
        );

        Ok(())
    }

    #[allow(unused)]
    pub fn add_column_values(&mut self, col_idx: usize, values: Vec<Data>) -> Result<(), String> {
        let column = self
            .columns
            .get_mut(col_idx)
            .ok_or_else(|| format!("Column {col_idx} doesn't exist"))?;

        column.values.extend(values);

        Ok(())
    }

    #[allow(unused)]
    pub fn replace_column_values(
        &mut self,
        col_idx: usize,
        values: Vec<Data>,
    ) -> Result<(), String> {
        let column = self
            .columns
            .get_mut(col_idx)
            .ok_or_else(|| format!("Column {col_idx} doesn't exis"))?;

        let len_in = values.len();
        let len_ex = column.values.len();

        if len_in != len_ex {
            return Err(format!(
                "Unequal vector length: new vector {len_in:?} is not the same size as the existing vector {len_ex:?}"
            ));
        }

        for value in &values {
            if !column.column_type.matches(value) {
                return Err(format!("Invalid data type for Column '{}'", column.heading));
            }
        }

        column.values = values;

        Ok(())
    }

    #[allow(unused)]
    pub fn get_column_headings(&self) -> Vec<String> {
        let mut vec_headings: Vec<String> = vec![];
        for column in self.columns() {
            vec_headings.push(column.heading().to_string());
        }
        vec_headings
    }

    #[allow(unused)]
    pub fn get_column_heading(&self, n: usize) -> String {
        let mut idx: usize = 0;
        let mut heading = "".to_string();
        for column in self.columns() {
            if idx == n {
                heading = column.heading().to_string();
            }
            idx += 1;
        }
        heading
    }

    #[allow(unused)]
    pub fn get_column(&self, heading: &str) -> Option<&Column> {
        self.columns.iter().find(|c| c.heading == heading)
    }

    #[allow(unused)]
    pub fn get_column_mut(&mut self, heading: &str) -> Option<&mut Column> {
        self.columns.iter_mut().find(|c| c.heading == heading)
    }

    #[allow(unused)]
    pub fn get_value(&self, heading: &str, row: usize) -> Option<&Data> {
        self.get_column(heading)?.values().get(row)
    }

    #[allow(unused)]
    pub fn set_value(&mut self, heading: &str, row: usize, value: Data) -> Result<(), String> {
        let column = self
            .get_column_mut(heading)
            .ok_or_else(|| format!("Column '{heading}' not  found"))?;

        if column.values.len() == 0 {
            return Err(format!("Values vector is empty (len = 0)!"));
        }

        if !column.column_type.matches(&value) {
            return Err(format!(
                "Invalid data type for Column '{}': expected {:?}, received {:?}",
                heading, column.column_type, value
            ));
        }

        if row >= column.values.len() {
            return Err(format!("Row {row} doesn't exist {}", column.values.len()));
        }

        column.values[row] = value;

        Ok(())
    }

    #[allow(unused)]
    pub fn push_value(&mut self, heading: &str, value: Data) -> Result<(), String> {
        self.get_column_mut(heading)
            .ok_or_else(|| format!("Column '{heading}' not found"))?
            .push(value)
    }
}

pub struct HeadingBuilder {
    columns: Vec<(String, ColumnType)>,
    max_count: usize,
}

impl HeadingBuilder {
    pub fn new(max_count: usize) -> Self {
        Self {
            columns: Vec::with_capacity(max_count),
            max_count,
        }
    }

    #[allow(unused)]
    pub fn max_count(mut self, max_count: usize) -> Self {
        self.max_count = max_count;
        self
    }

    pub fn add<S: Into<String>>(mut self, heading: S, column_type: ColumnType) -> Self {
        self.columns.push((heading.into(), column_type));
        self
    }

    pub fn build(self) -> Result<Heading, String> {
        if self.columns.len() != self.max_count {
            return Err(format!(
                "Expected: {}, received: {}",
                self.max_count,
                self.columns.len()
            ));
        }

        let columns = self
            .columns
            .into_iter()
            .map(|(heading, column_type)| Column {
                heading,
                column_type,
                values: Vec::new(),
            })
            .collect();

        Ok(Heading { columns })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use calamine::{Data, DataType};

    fn create_test_builder() -> HeadingBuilder {
        HeadingBuilder::new(2)
            .add("Nr", ColumnType::Integer)
            .add("Name", ColumnType::Text)
    }

    fn create_test_table() -> Heading {
        let mut table = HeadingBuilder::new(3)
            .add("Name", ColumnType::Text)
            .add("Stunden", ColumnType::Number)
            .add("Preis", ColumnType::Euro)
            .build()
            .unwrap();
        table
            .add_column_values(0, vec![Data::String("Mustermann".to_string())])
            .unwrap();
        table.add_column_values(1, vec![Data::Float(7.5)]).unwrap();
        table
            .add_column_values(2, vec![Data::Float(80.75)])
            .unwrap();
        table
    }

    #[test]
    fn test_struct_column() {
        let col = Column {
            heading: "Preis".to_string(),
            column_type: ColumnType::Euro,
            values: vec![Data::Float(1.25), Data::Float(4.37)],
        };

        assert_eq!(col.heading(), "Preis");
        assert_eq!(col.col_type(), &ColumnType::Euro);
        assert_eq!(col.values(), vec![Data::Float(1.25), Data::Float(4.37)])
    }

    #[test]
    fn test_build_heading_success() {
        let heading = HeadingBuilder::new(2)
            .add("Name", ColumnType::Text)
            .add("Preis", ColumnType::Euro)
            .build();

        assert!(heading.is_ok());

        let heading = heading.unwrap();

        assert_eq!(heading.columns().len(), 2);
        assert_eq!(heading.columns()[1].heading(), "Preis");
    }

    #[test]
    fn test_build_heading_wrong_column_count() {
        let result = HeadingBuilder::new(2).add("Name", ColumnType::Text).build();

        assert!(result.is_err());
    }

    #[test]
    fn test_add_heading() {
        let heading = create_test_builder()
            .max_count(3)
            .add("Datum", ColumnType::Date);

        let result = heading.build();

        assert!(result.is_ok());

        let heading = result.unwrap();
        assert_eq!(heading.columns().len(), 3);
        assert_eq!(heading.columns()[2].heading(), "Datum");
        assert_eq!(heading.columns()[2].col_type(), &ColumnType::Date);
    }

    #[test]
    fn test_add_colum() {
        let mut table = create_test_table();

        table.add_column("Datum", ColumnType::Date).unwrap();

        let headings = table.get_column_headings();

        assert_eq!(headings[3], "Datum".to_string(),);
    }

    #[test]
    fn test_add_column_at_position() {
        let mut table = create_test_table();

        table.add_column_at("Datum", ColumnType::Date, 1).unwrap();

        let headings = table.get_column_headings();

        assert_eq!(
            headings,
            vec![
                "Name".to_string(),
                "Datum".to_string(),
                "Stunden".to_string(),
                "Preis".to_string(),
            ]
        );
    }

    #[test]
    fn test_column_mut() {
        let mut table = create_test_table();
        let cols = table.column_mut(2).unwrap();

        assert_eq!(cols.heading, "Preis");

        cols.heading = "Gehalt".to_string();
        assert_eq!(cols.heading, "Gehalt");
    }

    #[test]
    fn test_add_column_values() {
        let mut table = create_test_builder().build().unwrap();

        table
            .add_column_values(0, vec![Data::Int(1), Data::Int(2), Data::Int(3)])
            .unwrap();
        let col_vals = table.get_column("Nr").unwrap();

        assert_eq!(col_vals.values().len(), 3);
        assert_eq!(col_vals.values()[0], Data::Int(1));
        assert_eq!(col_vals.values()[1], Data::Int(2));
        assert_eq!(col_vals.values()[2], Data::Int(3));
    }

    #[test]
    fn test_replace_column_values() {
        let mut table = create_test_table();
        table.add_column_values(1, vec![Data::Float(3.4)]).unwrap();

        table
            .replace_column_values(1, vec![Data::Float(1.1), Data::Float(2.9)])
            .unwrap();
        let col_vals = table.get_column("Stunden").unwrap();

        assert_eq!(col_vals.values().len(), 2);
        assert_eq!(col_vals.values()[0], Data::Float(1.1));
        assert_eq!(col_vals.values()[1], Data::Float(2.9));
    }

    #[test]
    fn test_replace_column_values_wrong_length() {
        let mut table = create_test_table();

        let err = table
            .replace_column_values(1, vec![Data::Float(1.1), Data::Float(2.9)])
            .unwrap_err();

        assert!(
            err.contains("Ungleiche Vektorlänge"),
            "Fehlermeldung lautet: {err}"
        );
    }

    #[test]
    fn test_get_column_heading() {
        let table = create_test_table();

        assert_eq!(table.get_column_heading(1), "Stunden");
    }

    #[test]
    fn test_get_column_headings() {
        let table = create_test_builder().build().unwrap();
        let headings = table.get_column_headings();

        assert_eq!(headings, vec!["Nr", "Name"]);
    }

    #[test]
    fn test_get_value() {
        let table = create_test_table();

        assert_eq!(
            table.get_value("Stunden", 0).unwrap().as_f64().unwrap(),
            7.5
        );
    }

    #[test]
    fn test_set_value_success() {
        let mut table = create_test_table();

        table.set_value("Preis", 0, Data::Float(19.95)).unwrap();

        let value = table.get_value("Preis", 0).unwrap();

        assert_eq!(value.as_f64().unwrap(), 19.95);
    }

    #[test]
    fn test_set_value_wrong_type() {
        let mut table = create_test_table();

        let err = table
            .set_value("Preis", 0, Data::String("Name".to_string()))
            .unwrap_err();

        assert!(
            err.contains("Ungültiger Datentyp"),
            "Fehlermeldung lautet: {err}"
        );
    }

    #[test]
    fn test_push_value_success() {
        let mut table = create_test_table();

        table
            .push_value("Name", Data::String("Meier".to_string()))
            .unwrap();

        assert_eq!(table.get_value("Name", 1).unwrap(), "Meier");
    }

    #[test]
    fn test_push_value_wrong_type() {
        let mut table = create_test_table();

        let err = table.push_value("Name", Data::Float(3.4)).unwrap_err();

        assert!(
            err.contains("Ungültiger Datentyp"),
            "Fehlermeldung lautet: {err}"
        );
    }
}
