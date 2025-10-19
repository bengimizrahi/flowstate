use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ops::{Add, Sub, SubAssign};

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc, Duration, Datelike};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

pub type TeamName = String;
pub type ResourceName = String;
pub type LabelName = String;
pub type FilterName = String;
pub type Days = u64;
pub type Fraction = u8;
pub type TaskId = u64;

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
        
        if total_fraction >= 100 {
            TaskDuration {
                days: total_days + (total_fraction / 100) as u64,
                fraction: total_fraction % 100,
            }
        } else {
            TaskDuration {
                days: total_days,
                fraction: total_fraction,
            }
        }
    }
}

impl Sub for TaskDuration {
    type Output = TaskDuration;

    fn sub(self, other: TaskDuration) -> TaskDuration {
        let self_total = self.days * 100 + self.fraction as u64;
        let other_total = other.days * 100 + other.fraction as u64;

        if other_total <= self_total {
            let result_total = self_total - other_total;
            TaskDuration {
                days: result_total / 100,
                fraction: (result_total % 100) as u8,
            }
        } else {
            TaskDuration::zero()
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    NoOp,
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
    SwitchTeam{
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
        duration: TaskDuration,
    },
    UpdateTask{
        timestamp: DateTime<Utc>,
        id: TaskId,
        ticket: String,
        title: String,
        duration: TaskDuration,
    },
    DeleteTask{
        timestamp: DateTime<Utc>,
        id: TaskId,
    },
    PrioritizeTask{
        timestamp: DateTime<Utc>,
        task_id: TaskId,
        to_top: bool,
    },
    DeprioritizeTask{
        timestamp: DateTime<Utc>,
        task_id: TaskId,
        to_bottom: bool,
    },
    ChangeTaskPriority{
        timestamp: DateTime<Utc>,
        task_id: TaskId,
        delta: i32,
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
    AddLabelToTask{
        timestamp: DateTime<Utc>,
        task_id: TaskId,
        label_name: LabelName,
    },
    RemoveLabelFromTask{
        timestamp: DateTime<Utc>,
        task_id: TaskId,
        label_name: LabelName,
    },
    CreateModifyFilter{
        timestamp: DateTime<Utc>,
        name: FilterName,
        labels: Vec<LabelName>,
        is_favorite: bool,
    },
    RenameFilter{
        timestamp: DateTime<Utc>,
        old_name: FilterName,
        new_name: FilterName,
    },
    DeleteFilter{
        timestamp: DateTime<Utc>,
        name: FilterName,
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
        days: TaskDuration,
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
    CompoundCommand{
        timestamp: DateTime<Utc>,
        commands: Vec<Command>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommandRecord {
    undo_command: Command,
    redo_command: Command,
}

pub type TeamId = u64;
pub type ResourceId = u64;
pub type LabelId = u64;
pub type FilterId = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Absence {
    start_date: NaiveDate,
    duration: TaskDuration,
}

impl Absence {
    fn intersects(&self, other: &Self) -> bool {
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
    pub name: ResourceName,
    pub team_id: TeamId,
    pub assigned_tasks: Vec<TaskId>,
    pub watched_tasks: Vec<TaskId>,
    pub absences: Vec<Absence>,
}

impl Resource {
    fn new(name: ResourceName, team_id: TeamId) -> Self {
        Self {
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
    pub name: TeamName,
    pub resources: BTreeSet<ResourceId>,
}

impl Team {
    fn new(name: TeamName) -> Self {
        Self {
            name,
            resources: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: TaskId,
    pub ticket: String,
    pub title: String,
    pub duration: TaskDuration,
    pub label_ids: BTreeSet<LabelId>,
    pub assignee: Option<ResourceId>,
    pub watchers: BTreeSet<ResourceId>,
}

impl Task {
    fn new(_timestamp: DateTime<Utc>, id: TaskId, ticket: String, title: String, duration: TaskDuration) -> Self {
        Self {
            id,
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
    task_id: TaskId,
    date: NaiveDate,
    resource_id: ResourceId,
    pub fraction: Fraction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub date: NaiveDate,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    #[serde(skip)]
    filename: Option<String>,
    command_stack: Vec<CommandRecord>,
    num_commands_applied: usize,
    #[serde(skip)]
    flow_state: FlowState,
}

impl Project {
    pub fn new(yaml_filename: &str) -> Self {
        Self {
            filename: Some(yaml_filename.to_string()),
            command_stack: Vec::new(),
            num_commands_applied: 0,
            flow_state: FlowState::new(),
        }
    }

    pub fn load_from_yaml(yaml_filename: &str) -> Result<Self, String> {
        let mut file = File::open(yaml_filename).map_err(|e| format!("Failed to open YAML file: {}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| format!("Failed to read YAML file: {}", e))?;

        let (num_commands_applied, command_stack, ): (usize, Vec<CommandRecord>) =
            serde_yaml::from_str(&contents).map_err(|e| format!("Failed to deserialize YAML: {}", e))?;

        let flow_state = FlowState::from_commands(&command_stack.iter().take(num_commands_applied)
            .map(|record| record.redo_command.clone()).collect::<Vec<_>>())
            .expect("Failed to fast forward flow state");
        Ok(Self {
            filename: Some(yaml_filename.to_string()),
            command_stack,
            num_commands_applied,
            flow_state,
        })
    }

    pub fn save_to_yaml(&mut self) -> Result<(), String> {
        let data = (self.num_commands_applied, &self.command_stack);
        let yaml_string = serde_yaml::to_string(&data).map_err(|e| format!("Failed to serialize to YAML: {}", e))?;
        std::fs::write(self.filename.as_ref().unwrap(), yaml_string).map_err(|e| format!("Failed to write to file: {}", e))?;
        Ok(())
    }

    pub fn invoke_command(&mut self, command: Command) -> Result<(), String> {
        println!("Invoking command: {:?}", command);
        let undo_command = self.flow_state.execute_command_generate_inverse_and_rebuild_cache(command.clone())?;
        self.append_to_command_history(CommandRecord {
            undo_command,
            redo_command: command,
        });
        self.save_to_yaml()?;
        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), String> {
        if self.num_commands_applied == 0 {
            return Err("No commands to undo".to_string());
        }
        let command_record = &self.command_stack[self.num_commands_applied - 1];
        println!("Command for undo: {:?}", command_record.undo_command);
        self.flow_state.execute_command_generate_inverse_and_rebuild_cache(command_record.undo_command.clone())?;
        self.num_commands_applied -= 1;
        self.save_to_yaml()?;
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), String> {
        if self.num_commands_applied >= self.command_stack.len() {
            return Err("No commands to redo".to_string());
        }
        let command_record = &self.command_stack[self.num_commands_applied];
        println!("Command for redo: {:?}", command_record.redo_command);
        self.flow_state.execute_command_generate_inverse_and_rebuild_cache(command_record.redo_command.clone())?;
        self.num_commands_applied += 1;
        self.save_to_yaml()?;
        Ok(())
    }

    fn append_to_command_history(&mut self, command_record: CommandRecord) {
        if self.num_commands_applied < self.command_stack.len() {
            self.command_stack.truncate(self.num_commands_applied);
        }
        self.command_stack.push(command_record);
        self.num_commands_applied = self.command_stack.len();
    }

    pub fn flow_state(&self) -> &FlowState {
        &self.flow_state
    }

    pub fn flow_state_mut(&mut self) -> &mut FlowState {
        &mut self.flow_state
    }
}


#[derive(Debug, Clone)]
pub struct FlowState {
    pub teams: BTreeMap<TeamId, Team>,
    pub resources: BTreeMap<ResourceId, Resource>,
    pub tasks: BTreeMap<TaskId, Task>,
    pub labels: BTreeMap<LabelId, Label>,
    pub filters: BTreeMap<FilterId, Filter>,
    pub worklogs: HashMap<TaskId, HashMap<ResourceId, HashMap<NaiveDate, Worklog>>>,
    pub milestones: Vec<Milestone>,
    pub flow_state_cache: FlowStateCache,

    next_team_id: TeamId,
    next_resource_id: ResourceId,
    next_task_id: TaskId,
    next_label_id: LabelId,
    next_filter_id: FilterId,
}

impl FlowState {
    fn new() -> Self {
        let mut flow_state = Self {
            teams: BTreeMap::new(),
            resources: BTreeMap::new(),
            tasks: BTreeMap::new(),
            labels: BTreeMap::new(),
            filters: BTreeMap::new(),
            worklogs: HashMap::new(),
            milestones: Vec::new(),
            flow_state_cache: FlowStateCache::new(),

            next_team_id: 1,
            next_resource_id: 1,
            next_task_id: 1,
            next_label_id: 1,
            next_filter_id: 1,
        };
        flow_state.rebuild_cache();
        flow_state
    }

    fn from_commands(commands: &Vec<Command>) -> Result<Self, String> {
        let mut flow_state = FlowState::new();
        for command in commands {
            flow_state.execute_command_and_generate_inverse(command.clone())?;
        }
        flow_state.rebuild_cache();
        flow_state.reset_ids();
        Ok(flow_state)
    }

    fn execute_command_and_generate_inverse(&mut self, command: Command) -> Result<Command, String> {
        match command {
            Command::NoOp => Ok(Command::NoOp),
            Command::CreateTeam { timestamp, name} => {
                if self.teams.values().any(|team| team.name == name) {
                    return Err(format!("A team with the name '{}' already exists", name));
                }

                let team_id = self.next_team_id();
                self.teams.insert(team_id, Team::new(name.clone()));
                Ok(Command::DeleteTeam { timestamp, name })
            }
            Command::RenameTeam { timestamp, old_name, new_name } => {
                let team_id = self.teams.iter()
                    .find(|(_, team)| team.name == old_name)
                    .map(|(id, _)| *id);

                if team_id.is_none() {
                    return Err(format!("No team found with the name '{}'", old_name));
                }

                let team_id = team_id.unwrap();
                if self.teams.values().any(|team| team.name == new_name) {
                    return Err(format!("A team with the name '{}' already exists", new_name));
                }
                if let Some(team) = self.teams.get_mut(&team_id) {
                    team.name = new_name.clone();
                }
                Ok(Command::RenameTeam { timestamp, old_name: new_name, new_name: old_name })
            }
            Command::DeleteTeam { timestamp, name } => {
                let team_id = self.teams.iter()
                    .find(|(_, team)| team.name == name)
                    .map(|(id, _)| *id);

                if let Some(team_id) = team_id {
                    self.teams.remove(&team_id);
                } else {
                    return Err(format!("No team found with the name '{}'", name));
                }
                Ok(Command::CreateTeam { timestamp, name })
            }
            Command::CreateResource { timestamp, name, team_name } => {
                if self.resources.values().any(|res| res.name == name) {
                    return Err(format!("A resource with the name '{}' already exists", name));
                }

                let team_id = self.teams.iter()
                    .find(|(_, team)| team.name == team_name)
                    .map(|(id, _)| *id);

                if team_id.is_none() {
                    return Err(format!("No team found with the name '{}'", team_name));
                }

                let resource_id = self.next_resource_id();
                self.resources.insert(resource_id, Resource::new(name.clone(), team_id.unwrap()));

                if let Some(team) = self.teams.get_mut(&team_id.unwrap()) {
                    team.resources.insert(resource_id);
                }
                Ok(Command::DeleteResource { timestamp, name })
            }
            Command::RenameResource { timestamp, old_name, new_name } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == old_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", old_name));
                }

                let resource_id = resource_id.unwrap();
                if self.resources.values().any(|res| res.name == new_name) {
                    return Err(format!("A resource with the name '{}' already exists", new_name));
                }
                if let Some(resource) = self.resources.get_mut(&resource_id) {
                    resource.name = new_name.clone();
                }
                Ok(Command::RenameResource { timestamp, old_name: new_name, new_name: old_name })
            }
            Command::SwitchTeam { timestamp, resource_name, new_team_name } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == resource_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }

                let resource_id = resource_id.unwrap();

                let new_team_id = self.teams.iter()
                    .find(|(_, team)| team.name == new_team_name)
                    .map(|(id, _)| *id);

                if new_team_id.is_none() {
                    return Err(format!("No team found with the name '{}'", new_team_name));
                }

                let new_team_id = new_team_id.unwrap();

                let current_team_id = self.resources.get(&resource_id)
                    .map(|res| res.team_id);

                if let Some(current_team_id) = current_team_id {
                    let old_team_name = self.teams.get(&current_team_id)
                        .map(|team| team.name.clone())
                        .ok_or_else(|| "Current team not found".to_string())?;
                    
                    if let Some(current_team) = self.teams.get_mut(&current_team_id) {
                        current_team.resources.remove(&resource_id);
                    }
                    if let Some(new_team) = self.teams.get_mut(&new_team_id) {
                        new_team.resources.insert(resource_id);
                    }
    
                    if let Some(resource) = self.resources.get_mut(&resource_id) {
                        resource.team_id = new_team_id;
                    }
                    Ok(Command::SwitchTeam { timestamp, resource_name, new_team_name: old_team_name })
                } else {
                    Err("Resource has no current team".to_string())
                }
            }
            Command::DeleteResource { timestamp, name } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == name)
                    .map(|(id, _)| *id);

                if let Some(resource_id) = resource_id {
                    if let Some(resource) = self.resources.get(&resource_id) {
                        if !resource.assigned_tasks.is_empty() {
                            return Err(format!("Resource '{}' is assigned tasks and cannot be deleted", name));
                        }
                        if !resource.watched_tasks.is_empty() {
                            return Err(format!("Resource '{}' is watching tasks and cannot be deleted", name));
                        }
                        let has_worklogs = self.worklogs.values()
                            .any(|resource_map| resource_map.contains_key(&resource_id));
                        if has_worklogs {
                            return Err(format!("Resource '{}' has worklogs and cannot be deleted", name));
                        }
                    }
                    self.resources.remove(&resource_id)
                        .and_then(|resource| {
                            self.teams.get_mut(&resource.team_id)
                                .map(|team| {
                                    team.resources.remove(&resource_id);
                                    Command::CreateResource { 
                                        timestamp, 
                                        name: resource.name, 
                                        team_name: team.name.clone() 
                                    }
                                })
                        })
                        .ok_or_else(|| format!("Failed to create resource '{}'", name))
                } else {
                    return Err(format!("No resource found with the name '{}'", name));
                }
            },
            Command::CreateTask { timestamp, id, ticket, title, duration } => {
                let task = Task::new(timestamp, id, ticket, title, duration);
                self.tasks.insert(id, task);
                Ok(Command::DeleteTask { timestamp, id })
            }
            Command::UpdateTask { timestamp, id, ticket, title, duration } => {
                if let Some(task) = self.tasks.get_mut(&id) {
                    let original_ticket = task.ticket.clone();
                    let original_title = task.title.clone();
                    let original_duration = task.duration.clone();

                    task.ticket = ticket;
                    task.title = title;
                    task.duration = duration;
                    
                    Ok(Command::UpdateTask {
                        timestamp,
                        id,
                        ticket: original_ticket,
                        title: original_title,
                        duration: original_duration,
                    })
                } else {
                    return Err(format!("Task with id {} not found", id));
                }
            }
            Command::DeleteTask { timestamp, id } => {
                if let Some(task) = self.tasks.get(&id) {
                    if task.assignee.is_some() {
                        return Err(format!("Task with id {} is assigned to a resource and cannot be deleted", id));
                    }
                    if !task.watchers.is_empty() {
                        return Err(format!("Task with id {} has watchers and cannot be deleted", id));
                    }
                    let has_worklogs = self.worklogs.contains_key(&id);
                    if has_worklogs {
                        return Err(format!("Task with id {} has worklogs and cannot be deleted", id));
                    }
                    
                    // Clone the task data before removing it
                    let ticket = task.ticket.clone();
                    let title = task.title.clone();
                    let duration = task.duration.clone();
                    
                    self.tasks.remove(&id);
                    Ok(Command::CreateTask {
                        timestamp,
                        id,
                        ticket,
                        title,
                        duration,
                    })
                } else {
                    return Err(format!("Task with id {} not found", id));
                }
            }
            Command::PrioritizeTask { timestamp, task_id, to_top } => {
                if let Some(task) = self.tasks.get(&task_id) {
                    if let Some(assignee_id) = task.assignee {
                        if let Some(resource) = self.resources.get_mut(&assignee_id) {
                            let pos = resource.assigned_tasks.iter().position(|&id| id == task_id);
                            if let Some(pos) = pos {
                                if to_top {
                                    self.execute_command_and_generate_inverse(Command::ChangeTaskPriority { timestamp, task_id, delta: (-(pos as i32)) })
                                } else {
                                    self.execute_command_and_generate_inverse(Command::ChangeTaskPriority { timestamp, task_id, delta: if pos > 0 {-1} else {0}} )
                                }
                            } else {
                                Err(format!("Task with id {} is not assigned to any resource", task_id))
                            }
                        } else {
                            Err(format!("Resource with id {} not found", assignee_id))
                        }
                    } else {
                        Err(format!("Task with id {} is not assigned to any resource", task_id))
                    }
                } else {
                    Err(format!("Task with id {} not found", task_id))
                }
            }
            Command::DeprioritizeTask { timestamp, task_id, to_bottom} => {
                if let Some(task) = self.tasks.get(&task_id) {
                    if let Some(assignee_id) = task.assignee {
                        if let Some(resource) = self.resources.get_mut(&assignee_id) {
                            let pos = resource.assigned_tasks.iter().position(|&id| id == task_id);
                            if let Some(pos) = pos {
                                if to_bottom {
                                    let delta = (resource.assigned_tasks.len() - 1 - pos) as i32;
                                    self.execute_command_and_generate_inverse(Command::ChangeTaskPriority { timestamp, task_id, delta })
                                } else if pos < resource.assigned_tasks.len() - 1 {
                                    self.execute_command_and_generate_inverse(Command::ChangeTaskPriority { timestamp, task_id, delta: 1 })
                                } else {
                                    self.execute_command_and_generate_inverse(Command::ChangeTaskPriority { timestamp, task_id, delta: 0 })
                                }
                            } else {
                                Err(format!("Task with id {} is not assigned to any resource", task_id))
                            }
                        } else {
                            Err(format!("Resource with id {} not found", assignee_id))
                        }
                    } else {
                        Err(format!("Task with id {} is not assigned to any resource", task_id))
                    }
                } else {
                    Err(format!("Task with id {} not found", task_id))
                }
            }
            Command::ChangeTaskPriority { timestamp, task_id, delta } => {
                if let Some(task) = self.tasks.get(&task_id) {
                    if let Some(assignee_id) = task.assignee {
                        if let Some(resource) = self.resources.get_mut(&assignee_id) {
                            let pos = resource.assigned_tasks.iter().position(|&id| id == task_id);
                            if let Some(pos) = pos {
                                let new_pos = pos as i32 + delta;
                                if new_pos < 0 || new_pos >= resource.assigned_tasks.len() as i32 {
                                    return Err(format!("New position {} is out of bounds for task list of length {}", new_pos, resource.assigned_tasks.len()));
                                }
                                let new_pos = new_pos as usize;
                                resource.assigned_tasks.remove(pos);
                                resource.assigned_tasks.insert(new_pos, task_id);
                                Ok(Command::ChangeTaskPriority { timestamp, task_id, delta: -delta })
                            } else {
                                Err(format!("Task with id {} is not assigned to any resource", task_id))
                            }
                        } else {
                            Err(format!("Resource with id {} not found", assignee_id))
                        }
                    } else {
                        Err(format!("Task with id {} is not assigned to any resource", task_id))
                    }
                } else {
                    Err(format!("Task with id {} not found", task_id))
                }
            }
            Command::AssignTask { timestamp, task_id, resource_name } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == resource_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }

                let resource_id = resource_id.unwrap();

                if let Some(task) = self.tasks.get_mut(&task_id) {
                    match task.assignee {
                        Some(old_assignee_id) => {
                            let old_assignee_name = self.resources.get(&old_assignee_id)
                                .map(|res| res.name.clone())
                                .ok_or_else(|| format!("Old assignee resource with id {} not found", old_assignee_id))?;
                            if let Some(old_resource) = self.resources.get_mut(&old_assignee_id) {
                                old_resource.assigned_tasks.retain(|&x| x != task_id);
                            }
                            task.assignee = Some(resource_id);
                            if let Some(resource) = self.resources.get_mut(&resource_id) {
                                resource.assigned_tasks.insert(0, task_id);
                            }
                            Ok(Command::AssignTask { timestamp, task_id, resource_name: old_assignee_name })
                        }
                        None => {
                            task.assignee = Some(resource_id);
                            if let Some(resource) = self.resources.get_mut(&resource_id) {
                                resource.assigned_tasks.insert(0, task_id);
                            }
                            Ok(Command::UnassignTask { timestamp, task_id })
                        }
                    }
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::UnassignTask { timestamp, task_id } => {
                if let Some(task) = self.tasks.get_mut(&task_id) {
                    if let Some(old_assignee_id) = task.assignee.clone() {
                        if let Some(old_assignee_name) = self.resources.get(&old_assignee_id)
                            .map(|res| res.name.clone()) {
                                task.assignee = None;
                                if let Some(resource) = self.resources.get_mut(&old_assignee_id) {
                                    resource.assigned_tasks.retain(|&x| x != task_id);
                                }
                                Ok(Command::AssignTask { timestamp, task_id, resource_name: old_assignee_name })
                            } else {
                                Err(format!("Resource with id {} not found", old_assignee_id))
                            }
                    } else {
                        Err(format!("Task with id {} is not assigned to any resource", task_id))
                    }
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::AddWatcher { timestamp, task_id, resource_name } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == resource_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }

                let resource_id = resource_id.unwrap();

                if let Some(task) = self.tasks.get_mut(&task_id) {
                    task.watchers.insert(resource_id);
                    if let Some(resource) = self.resources.get_mut(&resource_id) {
                        resource.watched_tasks.insert(0, task_id);
                    }
                    Ok(Command::RemoveWatcher { timestamp, task_id, resource_name })
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::RemoveWatcher { timestamp, task_id, resource_name } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == resource_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }

                let resource_id = resource_id.unwrap();

                if let Some(task) = self.tasks.get_mut(&task_id) {
                    task.watchers.remove(&resource_id);
                    if let Some(resource) = self.resources.get_mut(&resource_id) {
                        resource.watched_tasks.retain(|&x| x != task_id);
                    }
                    Ok(Command::AddWatcher { timestamp, task_id, resource_name })
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::CreateLabel { timestamp, name } => {
                if self.labels.values().any(|label| label.name == name) {
                    return Err(format!("A label with the name '{}' already exists", name));
                }

                let label_id = self.next_label_id();
                self.labels.insert(label_id, Label { name: name.clone() });

                Ok(Command::DeleteLabel { timestamp, name })
            }
            Command::RenameLabel { timestamp, old_name, new_name } => {
                let label_id = self.labels.iter()
                    .find(|(_, label)| label.name == old_name)
                    .map(|(id, _)| *id);
                if label_id.is_none() {
                    return Err(format!("No label found with the name '{}'", old_name));
                }
                let label_id = label_id.unwrap();
                self.labels.insert(label_id, Label { name: new_name.clone() });
                Ok(Command::RenameLabel { timestamp, new_name, old_name })
            }

            Command::DeleteLabel { timestamp, name } => {
                let label_id = self.labels.iter()
                    .find(|(_, label)| label.name == name)
                    .map(|(id, _)| *id);
                if let Some(label_id) = label_id {
                    self.labels.remove(&label_id);
                    Ok(Command::CreateLabel { timestamp, name })
                } else {
                    return Err(format!("No label found with the name '{}'", name));
                }
            }
            Command::AddLabelToTask { timestamp, task_id, label_name } => {
                let label_id = self.get_label_id(&label_name);
                if label_id.is_none() {
                    return Err(format!("No label found with the name '{}'", label_name));
                }
                let label_id = label_id.unwrap();

                if let Some(task) = self.tasks.get_mut(&task_id) {
                    task.label_ids.insert(label_id);
                    Ok(Command::RemoveLabelFromTask { timestamp, task_id, label_name })
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::RemoveLabelFromTask { timestamp, task_id, label_name } => {
                let label_id = self.get_label_id(&label_name);
                if label_id.is_none() {
                    return Err(format!("No label found with the name '{}'", label_name));
                }
                let label_id = label_id.unwrap();

                if let Some(task) = self.tasks.get_mut(&task_id) {
                    task.label_ids.remove(&label_id);
                    Ok(Command::AddLabelToTask { timestamp, task_id, label_name })
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::CreateModifyFilter { timestamp, name, labels, is_favorite } => {
                let existing_filter_id = self.filters.iter()
                    .find(|(_, filter)| filter.name == name)
                    .map(|(id, _)| *id);

                let mut label_ids = BTreeSet::new();
                for label_name in &labels {
                    if let Some(label_id) = self.get_label_id(label_name) {
                        label_ids.insert(label_id);
                    } else {
                        return Err(format!("Label '{}' not found", label_name));
                    }
                }

                if let Some(filter_id) = existing_filter_id {
                    let old_labels = self.filters[&filter_id].labels.clone();
                    let old_label_names = old_labels.into_iter()
                        .filter_map(|id| self.labels.get(&id).map(|label| label.name.clone()))
                        .collect();
                    let old_is_favorite = self.filters[&filter_id].is_favorite;
                    self.filters.insert(filter_id, Filter { name: name.clone(), labels: label_ids, is_favorite });
                    Ok(Command::CreateModifyFilter { timestamp, name, labels: old_label_names, is_favorite: old_is_favorite })
                } else {
                    let filter_id = self.next_filter_id();
                    self.filters.insert(filter_id, Filter { name: name.clone(), labels: label_ids, is_favorite: false });
                    Ok(Command::DeleteFilter { timestamp, name })
                }
            }
            Command::RenameFilter { timestamp, old_name, new_name } => {
                let filter_id = self.filters.iter()
                    .find(|(_, filter)| filter.name == old_name)
                    .map(|(id, _)| *id);
                if filter_id.is_none() {
                    return Err(format!("No filter found with the name '{}'", old_name));
                }
                let filter_id = filter_id.unwrap();
                if self.filters.values().any(|filter| filter.name == new_name) {
                    return Err(format!("A filter with the name '{}' already exists", new_name));
                }
                if let Some(filter) = self.filters.get_mut(&filter_id) {
                    filter.name = new_name.clone();
                }
                Ok(Command::RenameFilter { timestamp, old_name: new_name, new_name: old_name })
            }
            Command::DeleteFilter { timestamp, name } => {
                let filter_id = self.filters.iter()
                    .find(|(_, filter)| filter.name == name)
                    .map(|(id, _)| *id);
                if let Some(filter_id) = filter_id {
                    /* labels of the filter */
                    let filter = self.filters.get(&filter_id).cloned().unwrap();
                    let label_ids = filter.labels.clone();
                    let labels = label_ids.into_iter()
                        .filter_map(|id| self.labels.get(&id).map(|label| label.name.clone()))
                        .collect();
                    self.filters.remove(&filter_id);
                    Ok(Command::CreateModifyFilter { timestamp, name, labels, is_favorite: filter.is_favorite })
                } else {
                    return Err(format!("No filter found with the name '{}'", name));
                }
            }
            Command::SetWorklog { timestamp, task_id, date, resource_name, fraction } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == resource_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }

                let resource_id = resource_id.unwrap();

                if !self.tasks.contains_key(&task_id) {
                    return Err(format!("Task with id {} not found", task_id));
                }

                let worklog = Worklog {
                    task_id,
                    date,
                    resource_id,
                    fraction,
                };

                let previous_fraction = self.worklogs
                    .get(&task_id)
                    .and_then(|resource_map| resource_map.get(&resource_id))
                    .and_then(|date_map| date_map.get(&date))
                    .map(|w| w.fraction)
                    .unwrap_or(0);

                if fraction == 0 {
                    if previous_fraction == 0 {
                        return Err(format!("No worklog exists for task {} on {} for resource {}", task_id, date, resource_name));
                    }
                    
                    if let Some(resource_map) = self.worklogs.get_mut(&task_id) {
                        if let Some(date_map) = resource_map.get_mut(&resource_id) {
                            date_map.remove(&date);
                            if date_map.is_empty() {
                                resource_map.remove(&resource_id);
                            }
                        }
                        if resource_map.is_empty() {
                            self.worklogs.remove(&task_id);
                        }
                    }
                } else {
                    self.worklogs
                        .entry(task_id)
                        .or_insert_with(HashMap::new)
                        .entry(resource_id)
                        .or_insert_with(HashMap::new)
                        .insert(date, worklog);
                }

                Ok(Command::SetWorklog {
                    timestamp,
                    task_id,
                    date,
                    resource_name,
                    fraction: previous_fraction,
                })
            }
            Command::SetAbsence { timestamp, resource_name, start_date, days } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == resource_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }

                let resource_id = resource_id.unwrap();
                let absence = Absence {
                    start_date,
                    duration: days,
                };
                if let Some(absences) = self.resources.get_mut(&resource_id).map(|r| &mut r.absences) {
                    absences.retain(|a| !a.intersects(&absence));
                    if days > TaskDuration::zero() {
                        absences.push(absence);
                    }
                }
                Ok(Command::SetAbsence {
                    timestamp,
                    resource_name,
                    start_date,
                    days: if days > TaskDuration::zero() { TaskDuration::zero() } else { days },
                })
                /* This is not correct behavior! */
            }
            Command::AddMilestone { timestamp, title, date } => {
                let milestone = Milestone {
                    title,
                    date,
                };
                self.milestones.push(milestone.clone());
                Ok(Command::RemoveMilestone { timestamp, title: milestone.title })
            }
            Command::RemoveMilestone { timestamp, title } => {
                if let Some(pos) = self.milestones.iter().position(|m| m.title == title) {
                    let milestone = self.milestones.remove(pos);
                    Ok(Command::AddMilestone { timestamp, title: milestone.title, date: milestone.date })
                } else {
                    return Err(format!("No milestone found with the title '{}'", title));
                }
            }
            Command::CompoundCommand { timestamp, commands } => {
                let mut flow_state_clone = self.clone();
                let mut undo_commands = Vec::new();
                for cmd in commands {
                    let undo_cmd = flow_state_clone.execute_command_and_generate_inverse(cmd)?;
                    undo_commands.push(undo_cmd);
                }
                undo_commands.reverse();
                *self = flow_state_clone;
                Ok(Command::CompoundCommand { timestamp, commands: undo_commands })
            }
        }
    }

    fn execute_command_generate_inverse_and_rebuild_cache(&mut self, command: Command) -> Result<Command, String> {
        let undo_command = self.execute_command_and_generate_inverse(command)?;
        self.rebuild_cache();
        Ok(undo_command)
    }

    fn rebuild_cache(&mut self) {
        self.flow_state_cache = FlowStateCache::from(self);
    }

    fn reset_ids(&mut self) {
        self.next_team_id = self.teams.keys().max().map_or(1, |max_id| max_id + 1);
        self.next_resource_id = self.resources.keys().max().map_or(1, |max_id| max_id + 1);
        self.next_task_id = self.tasks.keys().max().map_or(1, |max_id| max_id + 1);
        self.next_label_id = self.labels.keys().max().map_or(1, |max_id| max_id + 1);
        self.next_filter_id = self.filters.keys().max().map_or(1, |max_id| max_id + 1);
    }
    
    fn get_team_name(&self, resource_name: &ResourceName) -> Option<TeamName> {
        self.resources.iter()
            .find(|(_, res)| res.name == *resource_name)
            .and_then(|(_, res)| self.teams.get(&res.team_id))
            .map(|team| team.name.clone())
    }

    fn get_label_id(&self, label_name: &LabelName) -> Option<LabelId> {
        self.labels.iter()
            .find(|(_, label)| label.name == *label_name)
            .map(|(id, _)| *id)
    }

    fn next_team_id(&mut self) -> TeamId {
        let id = self.next_team_id;
        self.next_team_id += 1;
        id
    }
    
    fn next_resource_id(&mut self) -> ResourceId {
        let id = self.next_resource_id;
        self.next_resource_id += 1;
        id
    }
    
    pub fn next_task_id(&mut self) -> TaskId {
        let id = self.next_task_id;
        self.next_task_id += 1;
        id
    }

    fn next_label_id(&mut self) -> LabelId {
        let id = self.next_label_id;
        self.next_label_id += 1;
        id
    }
    
    fn next_filter_id(&mut self) -> FilterId {
        let id = self.next_filter_id;
        self.next_filter_id += 1;
        id
    }

    pub fn cache(&self) -> &FlowStateCache {
        &self.flow_state_cache
    }
}

impl Default for FlowState {
    fn default() -> Self {
        FlowState::new()
    }
}

#[derive(Debug, Clone)]
pub struct AllocCursor {
    pub date: NaiveDate,
    pub alloced_amount: TaskDuration,
}

impl AllocCursor {
    fn new() -> Self {
        let mut cursor = AllocCursor {
            date: Utc::now().date_naive(),
            alloced_amount: TaskDuration { days: 0, fraction: 0 },
        };
        cursor.if_weekend_advance_to_monday();
        cursor
    }

    fn advance_to_next_working_day(&mut self) {
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

#[derive(Debug, Clone)]
pub struct FlowStateCache {
    start_date: NaiveDate,
    end_date: NaiveDate,
    pub date_to_milestones: BTreeMap<NaiveDate, Vec<Milestone>>,
    pub unassigned_tasks: Vec<TaskId>,
    pub unassigned_task_alloc_rendering: HashMap<TaskId, HashMap<NaiveDate, Fraction>>,
    pub task_alloc_rendering: HashMap<TaskId, HashMap<ResourceId, HashMap<NaiveDate, Fraction>>>,
    pub resource_absence_rendering: HashMap<ResourceId, HashMap<NaiveDate, Fraction>>,
    pub worklogs_on_others_tasks: HashMap<ResourceId, HashMap<NaiveDate, Fraction>>,
}

impl FlowStateCache {
    fn new() -> Self {
        FlowStateCache {
            start_date: Utc::now().date_naive(),
            end_date: Utc::now().date_naive(),
            date_to_milestones: BTreeMap::new(),
            unassigned_tasks: Vec::new(),
            unassigned_task_alloc_rendering: HashMap::new(),
            task_alloc_rendering: HashMap::new(),
            resource_absence_rendering: HashMap::new(),
            worklogs_on_others_tasks: HashMap::new(),
        }
    }

    fn from(flow_state: &FlowState) -> Self {
        let resource_absence_rendering: HashMap<ResourceId, HashMap<NaiveDate, Fraction>> = flow_state.resources.iter()
            .map(|(resource_id, resource)| {
                let absence_map = {
                    resource.absences.iter().fold(HashMap::new(), |mut acc, absence| {
                        let mut current_date = absence.start_date;
                        let mut remaining_days = absence.duration.days;
                        
                        while remaining_days > 0 {
                            if current_date.weekday() != chrono::Weekday::Sat && current_date.weekday() != chrono::Weekday::Sun {
                                acc.entry(current_date).or_insert(100);
                                remaining_days -= 1;
                            }
                            current_date = current_date + Duration::days(1);
                        }
                        
                        if absence.duration.fraction > 0 {
                            while current_date.weekday() == chrono::Weekday::Sat || current_date.weekday() == chrono::Weekday::Sun {
                                current_date = current_date + Duration::days(1);
                            }
                            acc.entry(current_date).or_insert(absence.duration.fraction);
                        }
                        acc
                    })
                };
                (*resource_id, absence_map)
            })
            .collect();
        let date_to_milestones = flow_state.milestones.iter()
            .fold(BTreeMap::new(), |mut acc, milestone| {
                acc.entry(milestone.date)
                    .or_insert_with(Vec::new)
                    .push(milestone.clone());
                acc
            });
        let unassigned_tasks : Vec<TaskId> = flow_state.tasks.iter()
            .filter(|(_, task)| task.assignee.is_none())
            .map(|(id, _)| *id)
            .collect();
        let total_worklogs: HashMap<TaskId, TaskDuration> = flow_state.worklogs.iter()
            .map(|(task_id, resource_map)| {
                let total = resource_map.values()
                    .flat_map(|date_map| date_map.values())
                    .fold(TaskDuration { days: 0, fraction: 0 }, |acc, worklog| {
                        acc + TaskDuration { days: 0, fraction: worklog.fraction }
                    });
                (*task_id, total)
            })
            .collect();
        let remaining_durations: HashMap<TaskId, TaskDuration> = flow_state.tasks.iter()
            .map(|(task_id, task)| {
                let total_worklog = total_worklogs.get(task_id)
                    .cloned()
                    .unwrap_or(TaskDuration { days: 0, fraction: 0 });
                (*task_id, TaskDuration::zero().max(task.duration.clone() - total_worklog))
            })
            .collect();

        let mut most_farther_alloc_date = chrono::Utc::now().date_naive();
        let mut task_alloc_rendering: HashMap<TaskId, HashMap<ResourceId, HashMap<NaiveDate, Fraction>>> = HashMap::new();
        for (resource_id, resource) in &flow_state.resources {
            let mut cursor = AllocCursor::new();
            for task_id in &resource.assigned_tasks {
                if let Some(_task) = flow_state.tasks.get(task_id) {
                    let mut remaining_alloc = remaining_durations.get(task_id)
                        .cloned()
                        .unwrap_or(TaskDuration { days: 0, fraction: 0 });
                    while remaining_alloc > (TaskDuration { days: 0, fraction: 0 }) {
                        let absence_for_current_day = resource_absence_rendering.get(resource_id)
                            .and_then(|absence_map| absence_map.get(&cursor.date))
                            .copied()
                            .unwrap_or(0);
                        let total_worklog_for_current_day = flow_state.worklogs.iter()
                            .filter_map(|(_, resource_map)| resource_map.get(resource_id))
                            .filter_map(|date_map| date_map.get(&cursor.date))
                            .map(|w| w.fraction)
                            .sum::<Fraction>();
                        let remaining_alloc_for_current_day = TaskDuration { days: 1, fraction: 0 } 
                            - cursor.alloced_amount
                            - TaskDuration { days: 0, fraction: total_worklog_for_current_day }
                            - TaskDuration { days: 0, fraction: absence_for_current_day };
                        let work_to_allocate = remaining_alloc.min(remaining_alloc_for_current_day);
                        if work_to_allocate > (TaskDuration { days: 0, fraction: 0 }) {
                            task_alloc_rendering.entry(*task_id).or_default()
                                .entry(*resource_id).or_default()
                                .insert(cursor.date, work_to_allocate.into());
                        }
                        remaining_alloc -= work_to_allocate;
                        if remaining_alloc == (TaskDuration { days: 0, fraction: 0 }) {
                            cursor += work_to_allocate;
                        } else {
                            cursor.advance_to_next_working_day();
                            most_farther_alloc_date = most_farther_alloc_date.max(cursor.date);
                        }
                    }
                }
            }
        }
        let mut unassigned_task_alloc_rendering: HashMap<TaskId, HashMap<NaiveDate, Fraction>> = HashMap::new();
        let today = Utc::now().date_naive();
        for task_id in &unassigned_tasks {
            let mut remaining_alloc = remaining_durations.get(task_id)
                .cloned()
                .unwrap_or(TaskDuration { days: 0, fraction: 0 });
            let mut date = today;
            while remaining_alloc > (TaskDuration { days: 0, fraction: 0 }) {
                while date.weekday() == chrono::Weekday::Sat || date.weekday() == chrono::Weekday::Sun {
                    date += Duration::days(1);
                }
                let work_to_allocate = remaining_alloc.min(TaskDuration { days: 1, fraction: 0 });
                unassigned_task_alloc_rendering.entry(*task_id).or_default().insert(date, work_to_allocate.into());
                remaining_alloc -= work_to_allocate;
                date += Duration::days(1);
            }
            most_farther_alloc_date = most_farther_alloc_date.max(date);
        }
        let mut worklogs_on_others_tasks: HashMap<ResourceId, HashMap<NaiveDate, Fraction>> = HashMap::new();
        for (task_id, task) in &flow_state.tasks {
            if let Some(resource_map) = flow_state.worklogs.get(task_id) {
                for (resource_id, date_map) in resource_map {
                    for (date, worklog) in date_map {
                        if task.assignee.is_none() || *resource_id != task.assignee.unwrap() {
                            *worklogs_on_others_tasks
                                .entry(*resource_id)
                                .or_insert_with(HashMap::new)
                                .entry(*date)
                                .or_insert(0) += worklog.fraction;
                        }
                    }
                }
            }
        }
        let mut start_date = flow_state.milestones.iter()
            .map(|m| m.date)
            .min()
            .unwrap_or(Utc::now().date_naive());
        start_date = start_date.min(flow_state.worklogs.iter()
            .flat_map(|(_, resource_map)| resource_map.values())
            .flat_map(|date_map| date_map.keys())
            .cloned()
            .min()
            .unwrap_or(start_date));
        start_date = start_date.min(flow_state.resources.iter()
            .flat_map(|(_, resource)| resource.absences.iter())
            .map(|absence| absence.start_date)
            .min()
            .unwrap_or(NaiveDate::MAX));
        start_date = start_date.checked_sub_signed(Duration::days(30))
            .unwrap_or(NaiveDate::MIN);
        let mut end_date = flow_state.milestones.iter()
            .map(|m| m.date)
            .max()
            .unwrap_or(Utc::now().date_naive());
        end_date = end_date.max(flow_state.worklogs.iter()
            .flat_map(|(_, resource_map)| resource_map.values())
            .flat_map(|date_map| date_map.keys())
            .cloned()
            .max()
            .unwrap_or(end_date));
        end_date = end_date.max(flow_state.resources.iter()
            .flat_map(|(_, resource)| resource.absences.iter())
            .map(|absence| absence.start_date)
            .max()
            .unwrap_or(NaiveDate::MIN));
        end_date = end_date.max(most_farther_alloc_date);
        end_date = end_date.checked_add_signed(Duration::days(30))
            .unwrap_or(NaiveDate::MAX);
        FlowStateCache {
            start_date,
            end_date,
            date_to_milestones,
            unassigned_tasks,
            unassigned_task_alloc_rendering,
            task_alloc_rendering,
            resource_absence_rendering,
            worklogs_on_others_tasks,
        }
    }

    pub fn day(&self, index: usize) -> NaiveDate {
        self.start_date + Duration::days(index as i64)
    }

    pub fn num_days(&self) -> usize {
        self.end_date.signed_duration_since(self.start_date).num_days() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_team() {
        let mut app = Project::new("test_project.yaml");
        let timestamp = Utc::now();
        let team_name = "Development".to_string();

        let result = app.invoke_command(Command::CreateTeam { timestamp, name: team_name });

        assert!(result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == "Development"));
    }

    #[test]
    fn test_undo_create_team() {
        let mut app = Project::new("test_project.yaml");
        let timestamp = Utc::now();
        let team_name = "Development".to_string();

        let result = app.invoke_command(Command::CreateTeam { timestamp, name: team_name });
        assert!(result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == "Development"));

        let undo_result = app.undo();
        assert!(undo_result.is_ok());
        assert!(!app.flow_state.teams.values().any(|team| team.name == "Development"));
    }

    #[test]
    fn test_undo_redo_create_team() {
        let mut app = Project::new("test_project.yaml");
        let timestamp = Utc::now();
        let team_name = "Development".to_string();

        let result = app.invoke_command(Command::CreateTeam { timestamp, name: team_name });
        assert!(result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == "Development"));

        let undo_result = app.undo();
        assert!(undo_result.is_ok());
        assert!(!app.flow_state.teams.values().any(|team| team.name == "Development"));

        let redo_result = app.redo();
        assert!(redo_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == "Development"));
    }

    #[test]
    fn test_create_rename_delete_team() {
        let mut app = Project::new("test_project.yaml");
        let timestamp = Utc::now();
        let team_name = "Development".to_string();
        let new_team_name = "Engineering".to_string();

        let create_result = app.invoke_command(Command::CreateTeam { timestamp, name: team_name.clone() });
        assert!(create_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == team_name));

        let rename_result = app.invoke_command(Command::RenameTeam { timestamp, old_name: team_name.clone(), new_name: new_team_name.clone() });
        assert!(rename_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == new_team_name));

        let delete_result = app.invoke_command(Command::DeleteTeam { timestamp, name: new_team_name.clone() });
        assert!(delete_result.is_ok());
        assert!(!app.flow_state.teams.values().any(|team| team.name == new_team_name));
    }

    #[test]
    fn test_create_rename_switch_team_delete_resource() {
        let mut app = Project::new("test_project.yaml");
        let timestamp = Utc::now();
        let team_name = "Development".to_string();
        let resource_name = "Alice".to_string();
        let new_team_name = "Engineering".to_string();

        let create_team_result = app.invoke_command(Command::CreateTeam { timestamp, name: team_name.clone() });
        assert!(create_team_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == team_name));

        let create_resource_result = app.invoke_command(Command::CreateResource { timestamp, name: resource_name.clone(), team_name: team_name.clone() });
        assert!(create_resource_result.is_ok());
        assert!(app.flow_state.resources.values().any(|res| res.name == resource_name));

        let rename_team_result = app.invoke_command(Command::RenameTeam { timestamp, old_name: team_name.clone(), new_name: new_team_name.clone() });
        assert!(rename_team_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == new_team_name));

        let switch_team_result = app.invoke_command(Command::SwitchTeam { timestamp, resource_name: resource_name.clone(), new_team_name: new_team_name.clone() });
        assert!(switch_team_result.is_ok());
        
        if let Some(resource) = app.flow_state.resources.get(&1) {
            assert_eq!(resource.team_id, 1); // Assuming the new team's ID is 1
        }

        let delete_resource_result = app.invoke_command(Command::DeleteResource { timestamp, name: resource_name.clone() });
        assert!(delete_resource_result.is_ok());
        assert!(!app.flow_state.resources.values().any(|res| res.name == resource_name));
    }

    #[test]
    fn test_undo_redo_create_task() {
        let mut app = Project::new("test_project");
        
        let timestamp = Utc::now();
        let task_id = app.flow_state_mut().next_task_id();
        let ticket = "TASK-123".to_string();
        let title = "Implement feature X".to_string();
        let duration = TaskDuration { days: 2, fraction: 50 };

        let create_task_result = app.invoke_command(
            Command::CreateTask {
                timestamp,
                id: task_id,
                ticket,
                title: title.clone(),
                duration,
            });
        assert!(create_task_result.is_ok());
        assert!(app.flow_state.tasks.values().any(|task| task.title == title));

        let undo_result = app.undo();
        assert!(undo_result.is_ok());
        assert!(!app.flow_state.tasks.values().any(|task| task.title == title));

        let redo_result = app.redo();
        assert!(redo_result.is_ok());
        assert!(app.flow_state.tasks.values().any(|task| task.title == title));
    }

    #[test]
    fn test_create_team_create_resource_save_to_yaml_load_from_yaml() {
        let mut app = Project::new("test_project.yaml");
        let timestamp = Utc::now();
        let team_name = "Development".to_string();
        let resource_name = "Alice".to_string();

        // Create a team
        let create_team_result = app.invoke_command(Command::CreateTeam { timestamp, name: team_name.clone() });
        assert!(create_team_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == team_name));

        // Create a resource in the team
        let create_resource_result = app.invoke_command(Command::CreateResource { timestamp, name: resource_name.clone(), team_name: team_name.clone() });
        assert!(create_resource_result.is_ok());
        assert!(app.flow_state.resources.values().any(|res| res.name == resource_name));

        // Save to YAML
        app.save_to_yaml().unwrap();

        // Load from YAML
        if let Ok(loaded_app) = Project::load_from_yaml("database.yaml") {
            // Verify loaded state
            assert!(loaded_app.flow_state.teams.values().any(|team| team.name == team_name));
            assert!(loaded_app.flow_state.resources.values().any(|res| res.name == resource_name));
        }

    }

    #[test]
    fn test_absence_intersections() {
        let a1 = Absence {
            start_date: NaiveDate::from_ymd_opt(2025, 8, 22).unwrap(),
            duration: TaskDuration { days: 1, fraction: 50 },
        };
        let a2 = Absence {
            start_date: NaiveDate::from_ymd_opt(2025, 8, 25).unwrap(),
            duration: TaskDuration { days: 0, fraction: 0 },
        };
        assert_eq!(a1.intersects(&a2), true);
    }

    #[test]
    fn test_alloc_cursor_add_assign_task_duration() {
        let mut cursor = AllocCursor::new();
        cursor += TaskDuration { days: 0, fraction: 50 };
        assert_eq!(cursor.alloced_amount, TaskDuration { days: 0, fraction: 50 });
    }
}
