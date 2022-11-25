use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Eq, PartialEq, Hash, Default, Ord, PartialOrd)]
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
