use crate::prelude::*;
pub use chrono::{
    self, DateTime as ChDateTime, Datelike, Duration, LocalResult, NaiveDate, NaiveTime, TimeZone,
    Timelike, Utc,
};
use std::fmt::{Display, Formatter};

pub trait DateTimeBehavior {
    fn as_date(&self) -> Option<&NaiveDate>;
    fn as_time(&self) -> Option<&NaiveTime>;
    fn as_date_time(&self) -> Option<&ChDateTime<chrono::Utc>>;

    // DateTime methods for accessing specific components of date or time values
    fn year(&self) -> Option<i32>;
    fn month(&self) -> Option<u32>;
    fn day(&self) -> Option<u32>;
    fn hour(&self) -> Option<u32>;
    fn minute(&self) -> Option<u32>;
    fn second(&self) -> Option<u32>;
    fn timestamp(&self) -> Option<i64>;
    fn timezone(&self) -> Option<Utc>;

    // Methods for formatting DateTime values as strings
    fn to_iso8601(&self) -> String;
    fn to_rfc3339(&self) -> String;

    // Methods for adding or subtracting a Duration to/from a DateTime value
    fn add_duration(&self, duration: Duration) -> Option<Self>
    where
        Self: Sized;
    fn subtract_duration(&self, duration: Duration) -> Option<Self>
    where
        Self: Sized;

    // Method for calculating the duration between two DateTime values
    fn duration_between(&self, other: &Self) -> Option<Duration>;

    fn from_ymd_opt(year: i32, month: u32, day: u32) -> Self;

    fn with_ymd_and_hms(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> Self;

    fn now() -> Self;
}

/// Enum representing a date, time, or date-time value.
///
/// # Variants
///
/// * `Date(NaiveDate)` - Represents a date without timezone information.
/// * `Time(NaiveTime)` - Represents a time without date and timezone information.
/// * `DateTime(ChDateTime<chrono::Utc>)` - Represents a date-time with timezone information.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DateTime {
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(ChDateTime<chrono::Utc>),
}

// Implementations of From trait to allow conversion from NaiveDate, NaiveTime, and ChDateTime<Utc>
impl From<NaiveDate> for DateTime {
    fn from(value: NaiveDate) -> Self {
        DateTime::Date(value)
    }
}

impl From<Value> for DateTime {
    fn from(value: Value) -> Self {
        match value {
            Value::DateTime(datetime) => datetime,
            _ => panic!("Cannot convert value to DateTime"),
        }
    }
}

impl From<NaiveTime> for DateTime {
    fn from(value: NaiveTime) -> Self {
        DateTime::Time(value)
    }
}

impl From<ChDateTime<chrono::Utc>> for DateTime {
    fn from(value: ChDateTime<chrono::Utc>) -> Self {
        DateTime::DateTime(value)
    }
}

// Implementations of From trait to allow conversion from LocalResult variants
impl From<LocalResult<NaiveDate>> for DateTime {
    fn from(value: LocalResult<NaiveDate>) -> Self {
        DateTime::Date(value.unwrap())
    }
}

impl From<LocalResult<NaiveTime>> for DateTime {
    fn from(value: LocalResult<NaiveTime>) -> Self {
        DateTime::Time(value.unwrap())
    }
}

impl From<LocalResult<ChDateTime<chrono::Utc>>> for DateTime {
    fn from(value: LocalResult<ChDateTime<chrono::Utc>>) -> Self {
        DateTime::DateTime(value.unwrap())
    }
}

// Implementation of From trait to allow conversion from &str
impl From<&str> for DateTime {
    fn from(value: &str) -> Self {
        match value.parse::<NaiveDate>() {
            Ok(date) => DateTime::Date(date),
            Err(_) => match value.parse::<NaiveTime>() {
                Ok(time) => DateTime::Time(time),
                Err(_) => match value.parse::<ChDateTime<chrono::Utc>>() {
                    Ok(datetime) => DateTime::DateTime(datetime),
                    Err(_) => panic!("Invalid date, time, or date-time format"),
                },
            },
        }
    }
}

// Implementation of From trait to allow conversion from i64
impl From<i64> for DateTime {
    fn from(value: i64) -> Self {
        DateTime::DateTime(ChDateTime::from_naive_utc_and_offset(
            chrono::DateTime::from_timestamp_nanos(value).naive_local(),
            Utc,
        ))
    }
}

/// Display implementation for DateTime.
impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DateTime::Date(value) => write!(f, "{}", value),
            DateTime::Time(value) => write!(f, "{}", value),
            DateTime::DateTime(value) => write!(f, "{}", value.to_rfc3339()),
        }
    }
}

// DateTime methods for accessing underlying NaiveDate, NaiveTime, or ChDateTime<Utc> values
impl DateTimeBehavior for DateTime {
    fn as_date(&self) -> Option<&NaiveDate> {
        match self {
            DateTime::Date(value) => Some(value),
            _ => None,
        }
    }

    fn as_time(&self) -> Option<&NaiveTime> {
        match self {
            DateTime::Time(value) => Some(value),
            _ => None,
        }
    }

    fn as_date_time(&self) -> Option<&ChDateTime<chrono::Utc>> {
        match self {
            DateTime::DateTime(value) => Some(value),
            _ => None,
        }
    }

    fn year(&self) -> Option<i32> {
        match self {
            DateTime::Date(date) => Some(date.year()),
            DateTime::DateTime(datetime) => Some(datetime.year()),
            _ => None,
        }
    }

    fn month(&self) -> Option<u32> {
        match self {
            DateTime::Date(date) => Some(date.month()),
            DateTime::DateTime(datetime) => Some(datetime.month()),
            _ => None,
        }
    }

    fn day(&self) -> Option<u32> {
        match self {
            DateTime::Date(date) => Some(date.day()),
            DateTime::DateTime(datetime) => Some(datetime.day()),
            _ => None,
        }
    }

    fn hour(&self) -> Option<u32> {
        match self {
            DateTime::Time(time) => Some(time.hour()),
            DateTime::DateTime(datetime) => Some(datetime.hour()),
            _ => None,
        }
    }

    fn minute(&self) -> Option<u32> {
        match self {
            DateTime::Time(time) => Some(time.minute()),
            DateTime::DateTime(datetime) => Some(datetime.minute()),
            _ => None,
        }
    }

    fn second(&self) -> Option<u32> {
        match self {
            DateTime::Time(time) => Some(time.second()),
            DateTime::DateTime(datetime) => Some(datetime.second()),
            _ => None,
        }
    }

    fn timestamp(&self) -> Option<i64> {
        match self {
            DateTime::DateTime(datetime) => Some(datetime.timestamp()),
            DateTime::Date(date) => Some(date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp()),
            DateTime::Time(time) => Some(
                NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .and_hms_opt(time.hour(), time.minute(), time.second())
                    .unwrap()
                    .and_utc()
                    .timestamp(),
            ),
        }
    }

    fn timezone(&self) -> Option<Utc> {
        match self {
            DateTime::DateTime(datetime) => Some(datetime.timezone()),
            _ => None,
        }
    }

    fn to_iso8601(&self) -> String {
        match self {
            DateTime::Date(date) => date.format("%Y-%m-%d").to_string(),
            DateTime::Time(time) => time.format("%H:%M:%S%.f").to_string(),
            DateTime::DateTime(datetime) => datetime.format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }

    fn to_rfc3339(&self) -> String {
        match self {
            DateTime::DateTime(datetime) => datetime.to_rfc3339(),
            _ => "".to_string(),
        }
    }

    fn add_duration(&self, duration: Duration) -> Option<Self> {
        match self {
            DateTime::Date(date) => Some(DateTime::Date(
                *date + chrono::Duration::days(duration.num_days()),
            )),
            DateTime::Time(_) => None, // Não é possível adicionar duração a um NaiveTime isolado
            DateTime::DateTime(datetime) => Some(DateTime::DateTime(*datetime + duration)),
        }
    }

    fn subtract_duration(&self, duration: Duration) -> Option<Self> {
        match self {
            DateTime::Date(date) => Some(DateTime::Date(
                *date - chrono::Duration::days(duration.num_days()),
            )),
            DateTime::Time(_) => None, // Não é possível subtrair duração de um NaiveTime isolado
            DateTime::DateTime(datetime) => Some(DateTime::DateTime(*datetime - duration)),
        }
    }

    fn duration_between(&self, other: &DateTime) -> Option<Duration> {
        match (self, other) {
            (DateTime::Date(date1), DateTime::Date(date2)) => {
                Some(Duration::days((*date2 - *date1).num_days()))
            }
            (DateTime::DateTime(dt1), DateTime::DateTime(dt2)) => Some(*dt2 - *dt1),
            _ => None, // Retornar None para combinações inválidas
        }
    }

    fn from_ymd_opt(year: i32, month: u32, day: u32) -> DateTime {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        DateTime::from(date)
    }

    fn with_ymd_and_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> DateTime {
        let datetime: chrono::LocalResult<chrono::DateTime<Utc>> =
            Utc.with_ymd_and_hms(year, month, day, hour, min, sec);
        DateTime::from(datetime)
    }

    fn now() -> DateTime {
        DateTime::from(Utc::now())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use chrono::{Duration, NaiveDate, TimeZone, Utc};

    #[test]
    fn test_add_duration() {
        let dt_date = DateTime::from_ymd_opt(2023, 4, 5);
        let dt_datetime = DateTime::with_ymd_and_hms(2023, 4, 5, 12, 34, 56);

        assert_eq!(
            dt_date.add_duration(Duration::days(1)),
            Some(DateTime::from(NaiveDate::from_ymd_opt(2023, 4, 6).unwrap()))
        );
        assert_eq!(
            dt_datetime.add_duration(Duration::days(1)),
            Some(DateTime::from(Utc.with_ymd_and_hms(2023, 4, 6, 12, 34, 56)))
        );
    }

    #[test]
    fn test_subtract_duration() {
        let date = NaiveDate::from_ymd_opt(2023, 4, 5).unwrap();
        let datetime = Utc.with_ymd_and_hms(2023, 4, 5, 12, 34, 56);

        let dt_date = DateTime::from(date);
        let dt_datetime = DateTime::from(datetime);

        assert_eq!(
            dt_date.subtract_duration(Duration::days(1)),
            Some(DateTime::from(NaiveDate::from_ymd_opt(2023, 4, 4).unwrap()))
        );
        assert_eq!(
            dt_datetime.subtract_duration(Duration::days(1)),
            Some(DateTime::from(Utc.with_ymd_and_hms(2023, 4, 4, 12, 34, 56)))
        );
    }

    #[test]
    fn test_duration_between() {
        let date1 = NaiveDate::from_ymd_opt(2023, 4, 5).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2023, 4, 6).unwrap();
        let datetime1 = Utc.with_ymd_and_hms(2023, 4, 5, 12, 34, 56);
        let datetime2 = Utc.with_ymd_and_hms(2023, 4, 6, 12, 34, 56);

        let dt_date1 = DateTime::from(date1);
        let dt_date2 = DateTime::from(date2);
        let dt_datetime1 = DateTime::from(datetime1);
        let dt_datetime2 = DateTime::from(datetime2);

        assert_eq!(
            dt_date1.duration_between(&dt_date2),
            Some(Duration::days(1))
        );
        assert_eq!(
            dt_datetime1.duration_between(&dt_datetime2),
            Some(Duration::days(1))
        );
    }

    #[test]
    fn test_timestamp() {
        let date = NaiveDate::from_ymd_opt(2023, 4, 5).unwrap();
        let datetime = Utc.with_ymd_and_hms(2023, 4, 5, 12, 34, 56);

        let dt_date = DateTime::from(date);
        let dt_datetime = DateTime::from(datetime);

        assert_eq!(
            dt_date.timestamp(),
            Some(date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp())
        );
        assert_eq!(dt_datetime.timestamp(), Some(datetime.unwrap().timestamp()));
    }

    #[test]
    fn test_timezone() {
        let datetime = Utc.with_ymd_and_hms(2023, 4, 5, 12, 34, 56);
        let dt_datetime = DateTime::from(datetime);

        assert_eq!(dt_datetime.timezone(), Some(Utc));
    }

    #[test]
    fn test_to_iso8601() {
        let date = NaiveDate::from_ymd_opt(2023, 4, 5).unwrap();
        let datetime = Utc.with_ymd_and_hms(2023, 4, 5, 12, 34, 56);

        let dt_date = DateTime::from(date);
        let dt_datetime = DateTime::from(datetime);

        assert_eq!(dt_date.to_iso8601(), "2023-04-05");
        assert_eq!(dt_datetime.to_iso8601(), "2023-04-05T12:34:56");
    }

    #[test]
    fn test_from_i64() {
        let timestamp_nanos = 1672539296000000000;
        let dt_datetime = DateTime::from(timestamp_nanos);

        assert_eq!(
            dt_datetime,
            DateTime::from(Utc.timestamp_nanos(timestamp_nanos))
        );
    }
}
