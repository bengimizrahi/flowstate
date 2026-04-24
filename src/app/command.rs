use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::NaiveDate;
use crate::app::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub timestamp: DateTime<Utc>,
    pub details: CommandDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandDetails {
    Void,
    CreateTeam{
        name: TeamName,
    },
    RenameTeam{
        old_name: TeamName,
        new_name: TeamName,
    },
    DeleteTeam{
        name: TeamName,
    },
    CreateResource{
        name: ResourceName,
        team_name: TeamName,
    },
    RenameResource{
        old_name: ResourceName,
        new_name: ResourceName,
    },
    SwitchTeam{
        resource_name: ResourceName,
        new_team_name: TeamName,
    },
    DeleteResource{
        name: ResourceName,
    },
    CreateTask{
        id: TaskId,
        ticket: String,
        title: String,
        duration: TaskDuration,
    },
    UpdateTask{
        id: TaskId,
        ticket: String,
        title: String,
        duration: TaskDuration,
    },
    DeleteTask{
        id: TaskId,
    },
    PrioritizeTask{
        task_id: TaskId,
        to_top: bool,
    },
    DeprioritizeTask{
        task_id: TaskId,
        to_bottom: bool,
    },
    ChangeTaskPriority{
        task_id: TaskId,
        delta: i32,
    },
    AssignTask{
        task_id: TaskId,
        resource_name: ResourceName,
    },
    UnassignTask{
        task_id: TaskId,
    },
    AddWatcher{
        task_id: TaskId,
        resource_name: ResourceName,
    },
    RemoveWatcher{
        task_id: TaskId,
        resource_name: ResourceName,
    },
    CreateLabel{
        name: String,
    },
    RenameLabel{
        old_name: String,
        new_name: String,
    },
    DeleteLabel{
        name: String,
    },
    AddLabelToTask{
        task_id: TaskId,
        label_name: LabelName,
    },
    RemoveLabelFromTask{
        task_id: TaskId,
        label_name: LabelName,
    },
    CreateModifyFilter{
        name: FilterName,
        labels: Vec<LabelName>,
        is_favorite: bool,
    },
    RenameFilter{
        old_name: FilterName,
        new_name: FilterName,
    },
    DeleteFilter{
        name: FilterName,
    },
    SetWorklog{
        task_id: TaskId,
        date: NaiveDate,
        resource_name: ResourceName,
        fraction: Fraction,
    },
    SetAbsence{
        resource_name: ResourceName,
        start_date: NaiveDate,
        days: TaskDuration,
    },
    AddMilestone{
        title: String,
        date: NaiveDate,
    },
    RemoveMilestone{
        title: String,
    },
    CompoundCommand{
        commands: Vec<Command>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRecord {
    pub undo_command: Command,
    pub redo_command: Command,
}