#[derive(Eq, PartialEq, Hash, Default)]
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

pub type GenericErrorResult<T> = Result<T, Box<dyn std::error::Error>>;
