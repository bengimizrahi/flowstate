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
    team_id: TeamId,
    assigned_tasks: BTreeSet<TaskId>,
    watched_tasks: BTreeSet<TaskId>,
    absences: Vec<Absence>,
}

impl Resource {
    fn new(name: ResourceName, team_id: TeamId) -> Self {
        Self {
            name,
            team_id,
            assigned_tasks: BTreeSet::new(),
            watched_tasks: BTreeSet::new(),
            absences: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Team {
    name: TeamName,
    resources: BTreeSet<ResourceId>,
}

impl Team {
    fn new(name: TeamName) -> Self {
        Self {
            name,
            resources: BTreeSet::new(),
        }
    }
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
            Command::RenameTeam { timestamp, old_name, new_name } => {
                self.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::RenameTeam {
                        timestamp: timestamp.clone(),
                        old_name: new_name.clone(),
                        new_name: old_name.clone(),
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
            Command::CreateResource { timestamp, name, team_name } => {
                self.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::DeleteResource {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::RenameResource { timestamp, old_name, new_name } => {
                self.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::RenameResource {
                        timestamp: timestamp.clone(),
                        old_name: new_name.clone(),
                        new_name: old_name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::SwitchTeam { timestamp, resource_name, new_team_name } => {
                self.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::SwitchTeam {
                        timestamp: timestamp.clone(),
                        resource_name: resource_name.clone(),
                        new_team_name: new_team_name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::DeleteResource { timestamp, name } => {
                let current_team_name = self.get_team_name(&name);
                self.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::CreateResource {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                        team_name: current_team_name.unwrap(),
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
                self.flow_state.teams.insert(team_id, Team::new(name.clone()));
            }
            Command::RenameTeam { old_name, new_name, .. } => {
                let team_id = self.flow_state.teams.iter()
                    .find(|(_, team)| team.name == old_name)
                    .map(|(id, _)| *id);

                if team_id.is_none() {
                    return Err(format!("No team found with the name '{}'", old_name));
                }

                let team_id = team_id.unwrap();
                if self.flow_state.teams.values().any(|team| team.name == new_name) {
                    return Err(format!("A team with the name '{}' already exists", new_name));
                }
                if let Some(team) = self.flow_state.teams.get_mut(&team_id) {
                    team.name = new_name;
                }
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
            Command::CreateResource { name, team_name, .. } => {
                if self.flow_state.resources.values().any(|res| res.name == name) {
                    return Err(format!("A resource with the name '{}' already exists", name));
                }

                let team_id = self.flow_state.teams.iter()
                    .find(|(_, team)| team.name == team_name)
                    .map(|(id, _)| *id);

                if team_id.is_none() {
                    return Err(format!("No team found with the name '{}'", team_name));
                }

                let resource_id = self.next_resource_id();
                self.flow_state.resources.insert(resource_id, Resource::new(name.clone(), team_id.unwrap()));

                if let Some(team) = self.flow_state.teams.get_mut(&team_id.unwrap()) {
                    team.resources.insert(resource_id);
                }
            }
            Command::RenameResource { old_name, new_name, .. } => {
                let resource_id = self.flow_state.resources.iter()
                    .find(|(_, res)| res.name == old_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", old_name));
                }

                let resource_id = resource_id.unwrap();
                if self.flow_state.resources.values().any(|res| res.name == new_name) {
                    return Err(format!("A resource with the name '{}' already exists", new_name));
                }
                if let Some(resource) = self.flow_state.resources.get_mut(&resource_id) {
                    resource.name = new_name;
                }
            }
            Command::SwitchTeam { resource_name, new_team_name, .. } => {
                let resource_id = self.flow_state.resources.iter()
                    .find(|(_, res)| res.name == resource_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }

                let resource_id = resource_id.unwrap();

                let new_team_id = self.flow_state.teams.iter()
                    .find(|(_, team)| team.name == new_team_name)
                    .map(|(id, _)| *id);

                if new_team_id.is_none() {
                    return Err(format!("No team found with the name '{}'", new_team_name));
                }

                let new_team_id = new_team_id.unwrap();

                let current_team_id = self.flow_state.resources.get(&resource_id)
                    .map(|res| res.team_id);

                if let Some(current_team_id) = current_team_id {
                    if let Some(current_team) = self.flow_state.teams.get_mut(&current_team_id) {
                        current_team.resources.remove(&resource_id);
                    }
                }

                if let Some(new_team) = self.flow_state.teams.get_mut(&new_team_id) {
                    new_team.resources.insert(resource_id);
                }

                if let Some(resource) = self.flow_state.resources.get_mut(&resource_id) {
                    resource.team_id = new_team_id;
                }
            }
            Command::DeleteResource { name, .. } => {
                let resource_id = self.flow_state.resources.iter()
                    .find(|(_, res)| res.name == name)
                    .map(|(id, _)| *id);

                if let Some(resource_id) = resource_id {
                    if let Some(resource) = self.flow_state.resources.remove(&resource_id) {
                        if let Some(team) = self.flow_state.teams.get_mut(&resource.team_id) {
                            team.resources.remove(&resource_id);
                        }
                    }
                } else {
                    return Err(format!("No resource found with the name '{}'", name));
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

    fn get_team_name(&self, resource_name: &ResourceName) -> Option<TeamName> {
        self.flow_state.resources.iter()
            .find(|(_, res)| res.name == *resource_name)
            .and_then(|(_, res)| self.flow_state.teams.get(&res.team_id))
            .map(|team| team.name.clone())
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

    #[test]
    fn test_create_rename_delete_team() {
        let mut app = Application::new();
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
        let mut app = Application::new();
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
}