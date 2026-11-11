use calamine::{Data, ExcelDateTime, ExcelDateTimeType};
use chrono::NaiveDate;
use log::debug;

fn date_serial(year: i32, month: u32, day: u32, is_1904: bool) -> Result<f64, String> {
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| format!("Invalid date: {year:04}-{month:02}-{day:02}"))?;

    let excel_epoch = if is_1904 {
        NaiveDate::from_ymd_opt(1904, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(1899, 12, 30).unwrap()
    };

    Ok((date - excel_epoch).num_days() as f64)
}

fn time_serial(hour: u32, minute: u32, second: u32) -> Result<f64, String> {
    if hour > 23 {
        return Err(format!("Invalid hour: {hour}. 0..=23 are permitted"));
    }

    if minute > 59 {
        return Err(format!("Invalid minute: {minute}. 0..=59 are permitted"));
    }

    if second > 59 {
        return Err(format!("Invalid second: {second}. 0..=59 are permitted"));
    }

    Ok((hour as f64 * 3600.0 + minute as f64 * 60.0 + second as f64) / 86400.0)
}


/// Create calamine::datatype::Data::DateTime from
/// 
/// # Arguments
/// 
/// year: i32
/// 
/// month: u32
/// 
/// day: u32
/// 
/// is_1904: bool
/// 
/// # Example
/// 
/// ```
/// 
/// use calamine::DataType;
/// use forecast::make_date;
/// let date = make_date(2026, 10, 23, false).unwrap();
/// assert!(date.is_datetime());
/// 
/// ```
/// 
///  # Panics
/// 
/// ```should_panic
/// // because month > 12
/// use forecast::make_date;
/// make_date(2026, 14, 2, false).unwrap();
/// ```
#[allow(unused)]
pub fn make_date(year: i32, month: u32, day: u32, is_1904: bool) -> Result<Data, String> {
    Ok(Data::DateTime(ExcelDateTime::new(
        date_serial(year, month, day, is_1904)?,
        ExcelDateTimeType::DateTime,
        is_1904,
    )))
}

/// Create calamine::datatype::Data::DateTime from
/// 
/// # Arguments
/// 
/// hour: i32
/// 
/// minute: u32
/// 
/// second: u32
/// 
/// is_1904: bool
/// 
/// # Example
/// 
/// ```
/// 
/// use calamine::DataType;
/// use forecast::make_time;
/// let time = make_time(16, 8, 53, false).unwrap();
/// assert!(time.is_datetime());
/// 
/// ```
/// 
///  # Panics
/// 
/// ```should_panic
/// // because hour > 23
/// use forecast::make_time;
/// make_time(28, 30, 0, false).unwrap();
/// ```
#[allow(unused)]
pub fn make_time(hour: u32, minute: u32, second: u32, is_1904: bool) -> Result<Data, String> {
    Ok(Data::DateTime(ExcelDateTime::new(
        time_serial(hour, minute, second)?,
        ExcelDateTimeType::DateTime,
        is_1904,
    )))
}

/// Create calamine::datatype::Data::DateTime from
/// 
///  # Arguments
/// 
/// year: i32
/// 
/// month: u32
/// 
/// day: u32
/// 
/// hour: i32
/// 
/// minute: u32
/// 
/// second: u32
/// 
/// is_1904: bool
/// 
/// # Example
/// 
/// ```
/// 
/// use calamine::DataType;
/// use forecast::make_datetime;
/// let datetime = make_datetime(2026, 10, 24, 13, 48, 5, false).unwrap();
/// assert!(datetime.is_datetime());
/// 
/// ```
#[allow(unused)]
pub fn make_datetime(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    is_1904: bool,
) -> Result<Data, String> {
    let days = date_serial(year, month, day, is_1904)?;
    let time_fraction = time_serial(hour, minute, second)?;

    Ok(Data::DateTime(ExcelDateTime::new(
        days + time_fraction,
        ExcelDateTimeType::DateTime,
        is_1904,
    )))
}

#[allow(unused)]
pub fn calc_excel_date(
    mut year: f64,
    mut month: f64,
    day: f64,
    hour: f64,
    minute: f64,
    second: f64,
    is_1904: bool,
) -> Data {
    // correction :leap year correction
    if month <= 2.0 {
        year -= 1.0;
        month += 12.0;
    }

    let days_since_1900 = ((365.0 * year) + (year / 4.0).floor() - (year / 100.0).floor()
        + (year / 400.0).floor()
        + ((153.0 * (month + 1.0)) / 5.0).floor()
        + day
        - 694022.0);

    let days_since_origin = if is_1904 {
        days_since_1900 - 1462.0
    } else {
        days_since_1900
    };
    debug!("days_since_origin: {}", days_since_origin);
    let daytime_fraction = (hour / 24.0 + minute / 1440.0 + second / 86400.0);
    debug!("daytime_fraction: {}", daytime_fraction);
    debug!("total: {}", days_since_origin + daytime_fraction);
    Data::DateTime(ExcelDateTime::new(
        days_since_origin + daytime_fraction,
        ExcelDateTimeType::DateTime,
        is_1904,
    ))
}

#[allow(unused)]
pub fn datetime_value(data: &Data) -> f64 {
        match data {
            Data::DateTime(dt) => dt.as_f64(),
            _ => panic!("Expected Data::DateTime"),
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use calamine::DataType;
    static TEST_EPSILON: f64 = 1e-10;

    fn init_logger() {
        let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
    } 

    /*fn datetime_value(data: &Data) -> f64 {
        match data {
            Data::DateTime(dt) => dt.as_f64(),
            _ => panic!("Expected Data::DateTime"),
        }
    }*/

    #[test]
    fn test_create_date() {
        init_logger();
        let date = make_date(2026, 11, 10, false).unwrap();
        let excel_value = calc_excel_date(2026.0, 11.0, 10.0, 0.0, 0.0, 0.0, false);

        assert_eq!(
            date, excel_value,
            "Error in excel-1900 date format creation"
        );

        let date_1904 = make_date(2026, 11, 10, true).unwrap();
        let excel_value_1904 = calc_excel_date(2026.0, 11.0, 10.0, 0.0, 0.0, 0.0, true);

        assert_eq!(
            date_1904, excel_value_1904,
            "Error in excel-1904 date format creation"
        );
    }

    #[test]
    fn test_create_time() {
        let time = make_time(14, 27, 43, false).unwrap();
        let excel_value = calc_excel_date(1899.0, 12.0, 30.0, 14.0, 27.0, 43.0, false);

        assert!(
            (datetime_value(&time) - datetime_value(&excel_value)).abs() < TEST_EPSILON,
            "Error in excel-1900 time format creation\nleft: {} \nright:{}",
            datetime_value(&time),
            datetime_value(&excel_value)
        );
        
        
        let time_1904 = make_time(14, 27, 43, true).unwrap();
        let excel_value_1904 = calc_excel_date(1904.0, 1.0, 1.0, 14.0, 27.0, 43.0, true);

        assert!(
            (datetime_value(&time_1904) - datetime_value(&excel_value_1904)).abs() < TEST_EPSILON,
            "Error in excel-1904 time format creation\nleft: {} \nright:{}",
            datetime_value(&time_1904),
            datetime_value(&excel_value_1904)
        );
    }

    #[test]
    fn test_create_datetime() {
        let datetime = make_datetime(2026, 11, 9, 8, 57, 12, false).unwrap();
        let excel_value = calc_excel_date(2026.0, 11.0, 9.0, 8.0, 57.0, 12.0, false);

        assert_eq!(
            datetime, excel_value,
            "Error in excel-1900 datetime format creation"
        );

        let datetime_1904 = make_datetime(2026, 11, 09, 8, 57, 12, true).unwrap();
        let excel_value_1904 = calc_excel_date(2026.0, 11.0, 9.0, 8.0, 57.0, 12.0, true);

        assert_eq!(
            datetime_1904, excel_value_1904,
            "Error in excel-1904 datetime format creation"
        );
    }

    #[test]
    fn test_invalid_date() {
        let err = make_datetime(2026, 2, 31, 12, 0, 0, false).unwrap_err();

        assert!(
            err.to_string().contains("Invalid date"),
            "Error: {}",
            err.to_string()
        );
    }

    #[test]
    fn test_invalid_time() {
        let err = make_datetime(2026, 3, 31, 12, 75, 0, false).unwrap_err();

        assert!(
            err.to_string().contains("Invalid minute"),
            "Error: {}",
            err.to_string()
        );
    }
}
