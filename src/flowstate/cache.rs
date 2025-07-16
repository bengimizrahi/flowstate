use chrono::{Local, Duration, NaiveDate};

#[derive(Debug, Clone)]
pub struct Level1Cache {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Level1Cache {
    pub fn new() -> Self {
        let current_date = Local::now().date_naive();
        Level1Cache {
            start_date: current_date,
            end_date: current_date,
        }
    }

    pub fn day(&self, offset: usize) -> NaiveDate {
        self.start_date + Duration::days(offset as i64)
    }

    pub fn num_days(&self) -> usize {
        self.end_date.signed_duration_since(self.start_date).num_days() as usize
    }
}
