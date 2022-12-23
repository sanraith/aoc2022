use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{self, Display},
    time::Duration,
};

#[derive(
    Eq, PartialEq, Hash, Default, Ord, PartialOrd, Serialize, Deserialize, Copy, Clone, Debug,
)]
pub struct YearDay {
    pub year: i32,
    pub day: u32,
}
impl YearDay {
    pub fn new(year: i32, day: u32) -> Self {
        YearDay { year, day }
    }
}

pub fn day_str(day: u32) -> String {
    format!("{:0>2}", day)
}

pub fn fmt_duration_s(d: &Duration) -> String {
    let scales = [
        (60_000_000, "min", 1),
        (1_000_000, "s", 2),
        (1_000, "ms", 0),
        (1, "us", 0),
    ];
    let micros = d.as_micros() as f64;
    for (scale, suffix, digits) in scales {
        let scaled = micros / scale as f64;
        if scaled >= 1.0 {
            let digits = (digits - scaled.log10() as i32).max(0);
            return format!("{:.*} {}", digits as usize, scaled, suffix);
        }
    }

    return "0 ms".to_owned();
}

pub fn fmt_duration(d: &Duration) -> String {
    let scales = [
        (60_000_000, "min", 2),
        (1_000_000, "s", 3),
        (1_000, "ms", 2),
        (1, "us", 0),
    ];
    let micros = d.as_micros() as f64;
    for (scale, suffix, digits) in scales {
        let scaled = micros / scale as f64;
        if scaled >= 1.0 {
            let digits = (digits - scaled.log10() as i32).max(0);
            return format!("{:.*} {}", digits as usize, scaled, suffix);
        }
    }

    return "0 ms".to_owned();
}

pub type DynError = Box<dyn std::error::Error>;
pub type GenericResult<T = ()> = Result<T, DynError>;

/// Simple error with a message
#[derive(Debug, Clone)]
pub struct MsgError<T: Clone + Display + fmt::Debug>(pub T);
impl<T: Clone + Display + fmt::Debug> Error for MsgError<T> {}
impl<T: Clone + Display + fmt::Debug> fmt::Display for MsgError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}
