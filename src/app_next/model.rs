use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime, Duration};
use chrono::Datelike;
use std::collections::BTreeSet;
use crate::app_next::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Absence {
    pub create_timestamp: DateTime<Utc>,
    pub start_date: NaiveDate,
    pub duration: TaskDuration,
}

impl Absence {
    pub fn intersects(&self, other: &Self) -> bool {
        let self_end_date = self.get_end_date();
        let other_end_date = other.get_end_date();
        self.start_date <= other_end_date.into() && other.start_date <= self_end_date.into()
    }

    fn get_end_date(&self) -> NaiveDateTime {
        let mut current_date = self.start_date;
        let mut remaining_days = self.duration.days;
        
        // Skip weekends for full days
        while remaining_days > 0 {
            while current_date.weekday() == chrono::Weekday::Sat || current_date.weekday() == chrono::Weekday::Sun {
                current_date = current_date + Duration::days(1);
            }
            remaining_days -= 1;
            current_date = current_date + Duration::days(1);
        }
        
        // Handle fraction part
        if self.duration.fraction > 0 {
            // Skip to next weekday if we're on a weekend
            while current_date.weekday() == chrono::Weekday::Sat || current_date.weekday() == chrono::Weekday::Sun {
            current_date = current_date + Duration::days(1);
            }
            
            // Convert fraction to hours (assuming 100 fraction = 24 hours)
            let fraction_hours = (self.duration.fraction as f64 / 100.0) * 24.0;
            let fraction_minutes = (fraction_hours * 60.0) as i64;
            
            // Start at beginning of day (midnight) and add the fraction time
            current_date.and_hms_opt(0, 0, 0).unwrap() + Duration::minutes(fraction_minutes)
        } else {
            current_date.and_hms_opt(0, 0, 0).unwrap()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    create_timestamp: DateTime<Utc>,
    pub name: ResourceName,
    pub team_id: TeamId,
    pub assigned_tasks: Vec<TaskId>,
    pub watched_tasks: Vec<TaskId>,
    pub absences: Vec<Absence>,
}

impl Resource {
    pub fn new(create_timestamp: DateTime<Utc>, name: ResourceName, team_id: TeamId) -> Self {
        Self {
            create_timestamp,
            name,
            team_id,
            assigned_tasks: Vec::new(),
            watched_tasks: Vec::new(),
            absences: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    create_timestamp: DateTime<Utc>,
    pub name: TeamName,
    pub resources: BTreeSet<ResourceId>,
}

impl Team {
    pub fn new(create_timestamp: DateTime<Utc>, name: TeamName) -> Self {
        Self {
            create_timestamp,
            name,
            resources: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: TaskId,
    create_timestamp: DateTime<Utc>,
    pub ticket: String,
    pub title: String,
    pub duration: TaskDuration,
    pub label_ids: BTreeSet<LabelId>,
    pub assignee: Option<ResourceId>,
    pub watchers: BTreeSet<ResourceId>,
}

impl Task {
    pub fn new(create_timestamp: DateTime<Utc>, id: TaskId, ticket: String, title: String, duration: TaskDuration) -> Self {
        Self {
            id,
            create_timestamp,
            ticket,
            title,
            duration,
            label_ids: BTreeSet::new(),
            assignee: None,
            watchers: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub name: String,
    pub labels: BTreeSet<LabelId>,
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worklog {
    pub create_timestamp: DateTime<Utc>,
    pub task_id: TaskId,
    pub date: NaiveDate,
    pub resource_id: ResourceId,
    pub fraction: Fraction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub date: NaiveDate,
    pub title: String,
}
