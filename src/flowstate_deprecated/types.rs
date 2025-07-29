use serde::{Serialize, Deserialize};

pub type TeamName = String;
pub type ResourceName = String;
pub type Days = u64;
pub type Fraction = u8;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Duration{
    pub days: Days,
    pub fraction: Fraction,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub u64);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Resource {
    pub name: ResourceName,
    pub team_name: TeamName,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: TaskId,
    pub ticket: String,
    pub title: String,
    pub duration: Duration,
    pub labels: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Worklog {
    pub date: String,
    pub task_id: TaskId,
    pub resource_name: ResourceName,
    pub fraction: Fraction,
}