use crate::app::*;
use chrono::NaiveDate;
use chrono::Datelike;
use chrono::Duration;

#[derive(Debug, Clone)]
pub struct AllocCursor {
    pub date: NaiveDate,
    pub alloced_amount: TaskDuration,
}

impl AllocCursor {
    pub fn new(date: NaiveDate) -> Self {
        let mut cursor = AllocCursor {
            date,
            alloced_amount: TaskDuration { days: 0, fraction: 0 },
        };
        cursor.if_weekend_advance_to_monday();
        cursor
    }

    pub fn advance_to_next_working_day(&mut self) {
        self.date = self.date + Duration::days(1);
        self.alloced_amount = TaskDuration { days: 0, fraction: 0 };
        while self.date.weekday() == chrono::Weekday::Sat || self.date.weekday() == chrono::Weekday::Sun {
            self.date = self.date + Duration::days(1);
        }
    }

    fn if_weekend_advance_to_monday(&mut self) {
        if self.date.weekday() == chrono::Weekday::Sat {
            self.date = self.date + Duration::days(2);
            self.alloced_amount = TaskDuration { days: 0, fraction: 0 };
        } else if self.date.weekday() == chrono::Weekday::Sun {
            self.date = self.date + Duration::days(1);
            self.alloced_amount = TaskDuration { days: 0, fraction: 0 };
        }
    }
}

impl std::ops::AddAssign<TaskDuration> for AllocCursor {
    fn add_assign(&mut self, other: TaskDuration) {
        let new_amount = self.alloced_amount + other;
        if new_amount.days > 0 {
            self.date = self.date + Duration::days(new_amount.days as i64);
            self.alloced_amount = TaskDuration { days: 0, fraction: new_amount.fraction };
        } else {
            self.alloced_amount = new_amount;
        }
        self.if_weekend_advance_to_monday();
    }
}