use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub, SubAssign};

pub type Days = u64;
pub type Fraction = u8;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TaskDuration {
    pub days: Days,
    pub fraction: Fraction,
}

impl From<TaskDuration> for Fraction {
    fn from(duration: TaskDuration) -> Self {
        (duration.days * 100) as Fraction + duration.fraction
    }
}

impl Add for TaskDuration {
    type Output = TaskDuration;

    fn add(self, other: TaskDuration) -> TaskDuration {
        let total_days = self.days + other.days;
        let total_fraction = self.fraction + other.fraction;

        TaskDuration {
            days: total_days + (total_fraction / 100) as u64,
            fraction: total_fraction % 100,
        }
    }
}

impl Sub for TaskDuration {
    type Output = TaskDuration;

    fn sub(self, other: TaskDuration) -> TaskDuration {
        let self_total = self.days * 100 + self.fraction as u64;
        let other_total = other.days * 100 + other.fraction as u64;
        let result_total = self_total.saturating_sub(other_total);

        TaskDuration {
            days: result_total / 100,
            fraction: (result_total % 100) as u8,
        }
    }
}

impl SubAssign for TaskDuration {
    fn sub_assign(&mut self, other: TaskDuration) {
        *self = *self - other;
    }
}

impl PartialEq for TaskDuration {
    fn eq(&self, other: &Self) -> bool {
        self.days == other.days && self.fraction == other.fraction
    }
}

impl Eq for TaskDuration {}

impl PartialOrd for TaskDuration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TaskDuration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.days.cmp(&other.days) {
            std::cmp::Ordering::Equal => self.fraction.cmp(&other.fraction),
            other => other,
        }
    }
}

impl TaskDuration {
    pub fn zero() -> Self {
        TaskDuration { days: 0, fraction: 0 }
    }
}