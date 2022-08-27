
pub mod concurrent {
    use chrono::NaiveDate;

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum DatePrecision { 
        Precise,
        Approximate,
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum DateType {
        Time,
        Day,
        Week,
        Month,
        Year,
        Decade,
        Century,
        Millennium,
    }

    pub struct HistoricalDate {
        pub date_type: DateType,
        pub precision: DatePrecision,
        
        pub start_date: Option<NaiveDate>,
        pub end_date: Option<NaiveDate>,
        pub range: Option<(NaiveDate, NaiveDate)>,
    }

    impl HistoricalDate {
        pub fn new(date_type: DateType, precision: DatePrecision) -> HistoricalDate {
            HistoricalDate {
                date_type,
                precision,
                start_date: None,
                end_date: None,
                range: None,
            }
        }
    }

    pub fn construct_date(year: i32, month: Option<u32>, day: Option<u32>) -> Result<NaiveDate, String> {
        if let Some(day_val) = day {
            if let Some(month_val) = month {
                Ok(NaiveDate::from_ymd(year, month_val, day_val))
            } else {
                Err(String::from("Cannot create a day-specific string without a month value."))
            }
        } else {
            if let Some(month_val) = month {
                Ok(NaiveDate::from_ymd(year, month_val, 1))
            } else {
                Ok(NaiveDate::from_ymd(year, 1, 1))
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use chrono::NaiveDate;
    use crate::concurrent::{HistoricalDate, construct_date};

    use super::*;
    #[test]
    fn can_create_historical_date() {
        let date = HistoricalDate::new(concurrent::DateType::Day, concurrent::DatePrecision::Precise);
        assert_eq!(date.start_date, None);
        assert_eq!(date.end_date, None);
        assert_eq!(date.range, None);
        assert_eq!(date.precision, concurrent::DatePrecision::Precise);
        assert_eq!(date.date_type, concurrent::DateType::Day);
    }

    #[test]
    fn i_can_construct_ce_ymd() {
        match construct_date(50, Some(3), Some(15)) {
          Ok(date) => {
            assert_eq!(date, NaiveDate::from_ymd(50, 3, 15));
          },
          Err(_error) => {}
        };
    }

    #[test]
    fn i_can_construct_bce_ymd() {
        match construct_date(-50, Some(3), Some(15)) {
            Ok(date) => {
              assert_eq!(date, NaiveDate::from_ymd(-50, 3, 15));
            },
            Err(_error) => {}
        };
    }

    #[test]
    fn i_can_construct_ce_ym() {
        match construct_date(50, Some(3), None) {
            Ok(date) => {
              assert_eq!(date, NaiveDate::from_ymd(50, 3, 1));
            },
            Err(_error) => {}
        };
    }

    #[test]
    fn i_can_construct_ce_y() {
        match construct_date(50, None, None) {
            Ok(date) => {
              assert_eq!(date, NaiveDate::from_ymd(50, 1, 1));
            },
            Err(_error) => {}
        };
    }

    #[test]
    fn i_get_an_error_if_i_try_day_no_month() {
        match construct_date(50, None, Some(15)) {
            Ok(_date) => {},
            Err(error) => {
                assert_eq!(error, String::from("Cannot create a day-specific string without a month value."));
            }
        };
    }
}