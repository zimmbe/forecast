use calamine::Data;

#[derive(Debug)]
pub enum ColumnType {
    Text,
    Integer, 
    Number,
    Date,
    Euro,
}

impl ColumnType {
    pub fn matches(&self, value: &Data) -> bool {
        match self {
            ColumnType::Text => matches!(value, Data::String(_)),

            ColumnType::Integer => matches!(
                value,
                Data::Int(_)
            ),

            ColumnType::Number => matches!(
                value,
                Data::Float(_) | Data::Int(_)
            ),

            ColumnType::Date => matches!(
                value,
                Data::DateTime(_)
            ),

            ColumnType::Euro => matches!(
                value,
                Data::Float(_) | Data::Int(_)
            ),
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
                    date.format("%d.%m.%Y").to_string()
                } else {
                    dt.to_string()
                }
            }
            _ => self.to_string(),
        }
    }
}

#[derive(Debug)]
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
        if !self.column_type.matches(&value) {
            return Err(format!(
                "Ungültiger Datentyp für Spalte '{}'",
                self.heading
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

#[derive(Debug)]
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

    pub fn add_column_values(&mut self, col_idx: usize, values: Vec<Data>) -> Result<(), String> {
        let column = self
            .columns
            .get_mut(col_idx)
            .ok_or_else(|| format!("Spalte {col_idx} existiert nicht"))?;

        column.values.extend(values);

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
    pub fn get_value(
        &self,
        heading: &str,
        row: usize,
    ) -> Option<&Data> {
        self.get_column(heading)?
            .values()
            .get(row)
    }

    #[allow(unused)]
    pub fn set_value(
        &mut self,
        heading: &str,
        row: usize,
        value: Data,
    ) -> Result<(), String> {
        let column = self
            .get_column_mut(heading)
            .ok_or_else(|| format!("Spalte '{heading}' nicht gefunden"))?;

        if !column.column_type.matches(&value) {
            return Err(format!(
                "Ungültiger Datentyp für Spalte '{}': erwartet {:?}, erhalten {:?}",
                heading,
                column.column_type,
                value
            ));
        }

        if row >= column.values.len() {
            return Err(format!("Zeile {row} existiert nicht"));
        }

        column.values[row] = value;

        Ok(())
    }

    #[allow(unused)]
    pub fn push_value(
        &mut self,
        heading: &str,
        value: Data,
    ) -> Result<(), String> {
        let column = self
            .get_column_mut(heading)
            .ok_or_else(|| format!("Spalte '{heading}' nicht gefunden"))?;

        column.push(value);

        Ok(())
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

    pub fn add<S: Into<String>>(mut self, heading: S, column_type: ColumnType) -> Self {
        self.columns.push((heading.into(), column_type));
        self
    }

    pub fn build(self) -> Result<Heading, String> {
        if self.columns.len() != self.max_count {
            return Err(format!(
                "Erwartet: {}, vorhanden: {}",
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
