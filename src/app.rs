use core::time;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use chrono::{DateTime, NaiveDate, Utc};

type TeamName = String;
type ResourceName = String;
type Days = u64;
type Fraction = u8;
type TaskId = u64;

#[derive(Debug, Clone)]
struct Duration{
    pub days: Days,
    pub fraction: Fraction,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
struct CommandRecord {
    undo_command: Command,
    redo_command: Command,
}

type TeamId = u64;
type ResourceId = u64;
type LabelId = u64;
type FilterId = u64;

#[derive(Debug)]
struct Absence {
    start_date: NaiveDate,
    duration: Duration,
}

#[derive(Debug)]
struct Resource {
    name: ResourceName,
    assigned_tasks: BTreeSet<TaskId>,
    watched_tasks: BTreeSet<TaskId>,
    absences: Vec<Absence>,
}

#[derive(Debug)]
struct Team {
    name: TeamName,
    resources: BTreeSet<ResourceId>,
}

#[derive(Debug)]
struct Task {
    id: TaskId,
    ticket: String,
    title: String,
    duration: Duration,
    labels: BTreeSet<LabelId>,
    assignee: Option<ResourceId>,
    watchers: BTreeSet<ResourceId>,
}

#[derive(Debug, Clone)]
struct Label {
    name: String,
}

#[derive(Debug)]
struct Filter {
    name: String,
    labels: BTreeSet<LabelId>,
}

#[derive(Debug)]
struct Worklog {
    task_id: TaskId,
    date: NaiveDate,
    resource_id: ResourceId,
    fraction: Fraction,
}

#[derive(Debug)]
struct Milestone {
    date: NaiveDate,
    title: String,
}

#[derive(Debug)]
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

    next_team_id: TeamId,
    next_resource_id: ResourceId,
    next_task_id: TaskId,
    next_label_id: LabelId,
    next_filter_id: FilterId,
}

#[derive(Debug)]
pub struct Application {
    command_stack: Vec<CommandRecord>,
    num_commands_applied: usize,
    flow_state: FlowState,
}

impl Application {
    pub fn new() -> Self {
        Self {
            command_stack: Vec::new(),
            num_commands_applied: 0,
            flow_state: FlowState {
                teams: BTreeMap::new(),
                resources: BTreeMap::new(),
                tasks: BTreeMap::new(),
                labels: BTreeMap::new(),
                filters: BTreeMap::new(),
                worklogs: HashMap::new(),
                milestones: Vec::new(),
                date_to_milestones: BTreeMap::new(),
                unassigned_tasks: BTreeSet::new(),
                resource_alloc_rendering: HashMap::new(),
                next_team_id: 1,
                next_resource_id: 1,
                next_task_id: 1,
                next_label_id: 1,
                next_filter_id: 1,
            },
        }
    }

    fn next_team_id(&mut self) -> TeamId {
        let id = self.flow_state.next_team_id;
        self.flow_state.next_team_id += 1;
        id
    }
    
    fn next_resource_id(&mut self) -> ResourceId {
        let id = self.flow_state.next_resource_id;
        self.flow_state.next_resource_id += 1;
        id
    }
    
    fn next_task_id(&mut self) -> TaskId {
        let id = self.flow_state.next_task_id;
        self.flow_state.next_task_id += 1;
        id
    }
    
    fn next_label_id(&mut self) -> LabelId {
        let id = self.flow_state.next_label_id;
        self.flow_state.next_label_id += 1;
        id
    }
    
    fn next_filter_id(&mut self) -> FilterId {
        let id = self.flow_state.next_filter_id;
        self.flow_state.next_filter_id += 1;
        id
    }

    fn invoke_command(&mut self, command: Command) -> Result<(), String> {
        match &command {
            Command::CreateTeam { timestamp, name } => {
                self.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::DeleteTeam {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::DeleteTeam { timestamp,name } => {
                self.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::CreateTeam {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                    },
                    redo_command: command,
                });
            }
            // Handle other commands similarly...
            _ => return Err("Command not implemented".to_string()),
        }
        Ok(())
    }

    fn execute_command(&mut self, command: Command) -> Result<(), String> {
        match command {
            Command::CreateTeam { name, .. } => {
                if self.flow_state.teams.values().any(|team| team.name == name) {
                    return Err(format!("A team with the name '{}' already exists", name));
                }

                let team_id = self.next_team_id();
                self.flow_state.teams.insert(team_id, Team {
                    name,
                    resources: BTreeSet::new(),
                });
            }
            Command::DeleteTeam { name, .. } => {
                let team_id = self.flow_state.teams.iter()
                    .find(|(_, team)| team.name == name)
                    .map(|(id, _)| *id);

                if let Some(team_id) = team_id {
                    self.flow_state.teams.remove(&team_id);
                } else {
                    return Err(format!("No team found with the name '{}'", name));
                }
            }
            // Handle other commands similarly...
            _ => return Err("Command not implemented".to_string()),
        }
        Ok(())
    }

    fn append_to_command_history(&mut self, command_record: CommandRecord) {
        if self.num_commands_applied < self.command_stack.len() {
            self.command_stack.truncate(self.num_commands_applied);
        }
        self.command_stack.push(command_record);
        self.num_commands_applied = self.command_stack.len();
    }

    fn undo(&mut self) -> Result<(), String> {
        if self.num_commands_applied == 0 {
            return Err("No commands to undo".to_string());
        }
        let command_record = &self.command_stack[self.num_commands_applied - 1];
        self.execute_command(command_record.undo_command.clone())?;
        self.num_commands_applied -= 1;
        Ok(())
    }

    fn redo(&mut self) -> Result<(), String> {
        if self.num_commands_applied >= self.command_stack.len() {
            return Err("No commands to redo".to_string());
        }
        let command_record = &self.command_stack[self.num_commands_applied];
        self.execute_command(command_record.redo_command.clone())?;
        self.num_commands_applied += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_team() {
        let mut app = Application::new();
        let timestamp = Utc::now();
        let team_name = "Development".to_string();

        let result = app.invoke_command(Command::CreateTeam { timestamp, name: team_name });

        assert!(result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == "Development"));
    }

    #[test]
    fn test_undo_create_team() {
        let mut app = Application::new();
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
        let mut app = Application::new();
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
}