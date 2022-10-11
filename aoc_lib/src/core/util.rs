// --- Collect all solutions ---
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
