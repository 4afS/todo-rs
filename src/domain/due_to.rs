use chrono::NaiveDate;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DueTo {
    pub date: NaiveDate,
}

impl DueTo {
    pub fn new(date: &NaiveDate) -> Self {
        DueTo { date: date.clone() }
    }
}
