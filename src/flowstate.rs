use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use chrono::{DateTime, NaiveDate, Utc};

type TeamName = String;
type ResourceName = String;
type Days = u64;
type Fraction = u8;
type TaskId = u64;

struct Duration{
    pub days: Days,
    pub fraction: Fraction,
}

enum Command {
    CreateTeam{
        timestamp: DateTime<Utc>,
        name: TeamName,
    },
    RenameTeam{
        timestamp: DateTime<Utc>,
        old_name: TeamName,
        new_name: TeamName,
    },
    DeleteTeam{
        timestamp: DateTime<Utc>,
        name: TeamName,
    },
    CreateResource{
        timestamp: DateTime<Utc>,
        name: ResourceName,
        team_name: TeamName,
    },
    RenameResource{
        timestamp: DateTime<Utc>,
        old_name: ResourceName,
        new_name: ResourceName,
    },
    SwitchResourceTeam{
        timestamp: DateTime<Utc>,
        resource_name: ResourceName,
        new_team_name: TeamName,
    },
    DeleteResource{
        timestamp: DateTime<Utc>,
        name: ResourceName,
    },
    CreateTask{
        timestamp: DateTime<Utc>,
        id: TaskId,
        ticket: String,
        title: String,
        duration: Duration,
        labels: Vec<Label>,
    },
    UpdateTask{
        timestamp: DateTime<Utc>,
        id: TaskId,
        ticket: String,
        title: String,
        duration: Duration,
        labels: Vec<Label>,
    },
    DeleteTask{
        timestamp: DateTime<Utc>,
        id: TaskId,
    },
    AssignTask{
        timestamp: DateTime<Utc>,
        task_id: TaskId,
        resource_name: ResourceName,
    },
    UnassignTask{
        timestamp: DateTime<Utc>,
        task_id: TaskId,
    },
    AddWatcher{
        timestamp: DateTime<Utc>,
        task_id: TaskId,
        resource_name: ResourceName,
    },
    RemoveWatcher{
        timestamp: DateTime<Utc>,
        task_id: TaskId,
        resource_name: ResourceName,
    },
    CreateLabel{
        timestamp: DateTime<Utc>,
        name: String,
    },
    RenameLabel{
        timestamp: DateTime<Utc>,
        old_name: String,
        new_name: String,
    },
    DeleteLabel{
        timestamp: DateTime<Utc>,
        name: String,
    },
    CreateFilter{
        timestamp: DateTime<Utc>,
        name: String,
        labels: Vec<LabelId>,
    },
    RenameFilter{
        timestamp: DateTime<Utc>,
        old_name: String,
        new_name: String,
    },
    DeleteFilter{
        timestamp: DateTime<Utc>,
        name: String,
    },
    SetWorklog{
        timestamp: DateTime<Utc>,
        task_id: TaskId,
        date: NaiveDate,
        resource_name: ResourceName,
        fraction: Fraction,
    },
    SetAbsence{
        timestamp: DateTime<Utc>,
        resource_name: ResourceName,
        start_date: NaiveDate,
        days: Duration,
    },
    AddMilestone{
        timestamp: DateTime<Utc>,
        title: String,
        date: NaiveDate,
    },
    RemoveMilestone{
        timestamp: DateTime<Utc>,
        title: String,
    },
}

struct CommandRecord {
    undo_command: Command,
    redo_command: Command,
}

type TeamId = u64;
type ResourceId = u64;
type LabelId = u64;
type FilterId = u64;

struct Absence {
    start_date: NaiveDate,
    duration: Duration,
}

struct Resource {
    name: ResourceName,
    assigned_tasks: BTreeSet<TaskId>,
    watched_tasks: BTreeSet<TaskId>,
    absences: Vec<Absence>,
}
struct Team {
    name: TeamName,
    resources: BTreeSet<ResourceId>,
}

struct Task {
    id: TaskId,
    ticket: String,
    title: String,
    duration: Duration,
    labels: BTreeSet<LabelId>,
    assignee: Option<ResourceId>,
    watchers: BTreeSet<ResourceId>,
}

struct Label {
    name: String,
}

struct Filter {
    name: String,
    labels: BTreeSet<LabelId>,
}

struct Worklog {
    task_id: TaskId,
    date: NaiveDate,
    resource_id: ResourceId,
    fraction: Fraction,
}

struct Milestone {
    date: NaiveDate,
    title: String,
}

struct FlowState {
    teams: BTreeMap<TeamId, Team>,
    resources: BTreeMap<ResourceId, Resource>,
    tasks: BTreeMap<TaskId, Task>,
    labels: BTreeMap<LabelId, Label>,
    filters: BTreeMap<FilterId, Filter>,
    worklogs: HashMap<TaskId, HashMap<ResourceId, HashMap<NaiveDate, Worklog>>>,
    milestones: Vec<Milestone>,
    date_to_milestones: BTreeMap<NaiveDate, Vec<Milestone>>,
    unassigned_tasks: BTreeSet<TaskId>,
    resource_alloc_rendering: HashMap<TaskId, HashMap<ResourceId, HashMap<NaiveDate, Fraction>>>,
}

struct Application {
    command_stack: Vec<CommandRecord>,
    applied_commands_count: usize,
    flow_state: FlowState,
}