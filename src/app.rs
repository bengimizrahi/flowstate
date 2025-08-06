use std::collections::{BTreeMap, BTreeSet, HashMap};

use chrono::{DateTime, NaiveDate, Utc};

type TeamName = String;
type ResourceName = String;
type LabelName = String;
type FilterName = String;
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
    },
    UpdateTask{
        timestamp: DateTime<Utc>,
        id: TaskId,
        ticket: String,
        title: String,
        duration: Duration,
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
        name: FilterName,
        labels: Vec<LabelName>,
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
    CompoundCommand{
        timestamp: DateTime<Utc>,
        commands: Vec<Command>,
    },
}

#[derive(Debug, Clone)]
struct CommandRecord {
    undo_command: Command,
    redo_command: Command,
}

type TeamId = u64;
type ResourceId = u64;
type LabelId = u64;
type FilterId = u64;
#[derive(Debug, Clone)]
struct Absence {
    start_date: NaiveDate,
    duration: Duration,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Task {
    id: TaskId,
    ticket: String,
    title: String,
    duration: Duration,
    label_ids: BTreeSet<LabelId>,
    assignee: Option<ResourceId>,
    watchers: BTreeSet<ResourceId>,
}

impl Task {
    fn new(timestamp: DateTime<Utc>, id: TaskId, ticket: String, title: String, duration: Duration) -> Self {
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

#[derive(Debug, Clone)]
struct Label {
    name: String,
}

#[derive(Debug, Clone)]
struct Filter {
    name: String,
    labels: BTreeSet<LabelId>,
}

#[derive(Debug, Clone)]
struct Worklog {
    task_id: TaskId,
    date: NaiveDate,
    resource_id: ResourceId,
    fraction: Fraction,
}

#[derive(Debug, Clone)]
struct Milestone {
    date: NaiveDate,
    title: String,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

    fn invoke_command(&mut self, command: Command) -> Result<(), String> {
        match &command {
            Command::CreateTeam { timestamp, name } => {
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::DeleteTeam {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::RenameTeam { timestamp, old_name, new_name } => {
                self.flow_state.execute_command(command.clone())?;
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
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::CreateTeam {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::CreateResource { timestamp, name, team_name } => {
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::DeleteResource {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::RenameResource { timestamp, old_name, new_name } => {
                self.flow_state.execute_command(command.clone())?;
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
                self.flow_state.execute_command(command.clone())?;
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
                let current_team_name = self.flow_state.get_team_name(&name);
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::CreateResource {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                        team_name: current_team_name.unwrap(),
                    },
                    redo_command: command,
                });
            }
            Command::CreateTask { timestamp, id, .. } => {
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::DeleteTask {
                        timestamp: timestamp.clone(),
                        id: *id,
                    },
                    redo_command: command,
                });
            }
            Command::UpdateTask { timestamp, id, ticket, title, duration } => {
                if let Some(task) = self.flow_state.tasks.get(&id) {
                    let original_ticket = task.ticket.clone();
                    let original_title = task.title.clone();
                    let original_duration = task.duration.clone();

                    self.flow_state.execute_command(command.clone())?;
                    self.append_to_command_history(CommandRecord {
                        undo_command: Command::UpdateTask {
                            timestamp: timestamp.clone(),
                            id: *id,
                            ticket: original_ticket,
                            title: original_title,
                            duration: original_duration,
                        },
                        redo_command: command,
                    });
                } else {
                    return Err(format!("Task with id {} not found", id));
                }
            }
            Command::DeleteTask { timestamp, id } => {
                if let Some(task) = self.flow_state.tasks.get(&id) {
                    let ticket = task.ticket.clone();
                    let title = task.title.clone();
                    let duration = task.duration.clone();

                    self.flow_state.execute_command(command.clone())?;
                    self.append_to_command_history(CommandRecord {
                        undo_command: Command::CreateTask {
                            timestamp: timestamp.clone(),
                            id: *id,
                            ticket,
                            title,
                            duration,
                        },
                        redo_command: command,
                    });
                } else {
                    return Err(format!("Task with id {} not found", id));
                }
            }
            Command::AssignTask { timestamp, task_id, resource_name } => {
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::UnassignTask {
                        timestamp: timestamp.clone(),
                        task_id: *task_id,
                    },
                    redo_command: command,
                });
            }
            Command::UnassignTask { timestamp, task_id } => {
                if let Some(task) = self.flow_state.tasks.get(&task_id) {
                    let resource_name = self.flow_state.resources.get(&task.assignee.unwrap())
                        .map(|res| res.name.clone())
                        .unwrap_or_default();

                    self.flow_state.execute_command(command.clone())?;
                    self.append_to_command_history(CommandRecord {
                        undo_command: Command::AssignTask {
                            timestamp: timestamp.clone(),
                            task_id: *task_id,
                            resource_name,
                        },
                        redo_command: command,
                    });
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::AddWatcher { timestamp, task_id, resource_name } => {
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::RemoveWatcher {
                        timestamp: timestamp.clone(),
                        task_id: *task_id,
                        resource_name: resource_name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::RemoveWatcher { timestamp, task_id, resource_name } => {
                if let Some(task) = self.flow_state.tasks.get(&task_id) {
                    self.flow_state.execute_command(command.clone())?;
                    self.append_to_command_history(CommandRecord {
                        undo_command: Command::AddWatcher {
                            timestamp: timestamp.clone(),
                            task_id: *task_id,
                            resource_name: resource_name.clone(),
                        },
                        redo_command: command,
                    });
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::CreateLabel { timestamp, name } => {
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::DeleteLabel {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::RenameLabel { timestamp, old_name, new_name } => {
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::RenameLabel {
                        timestamp: timestamp.clone(),
                        old_name: new_name.clone(),
                        new_name: old_name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::DeleteLabel { timestamp, name } => {
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::CreateLabel {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::CreateFilter { timestamp, name, labels } => {
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::DeleteFilter {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::RenameFilter { timestamp, old_name, new_name } => {
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::RenameFilter {
                        timestamp: timestamp.clone(),
                        old_name: new_name.clone(),
                        new_name: old_name.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::DeleteFilter { timestamp, name } => {
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::CreateFilter {
                        timestamp: timestamp.clone(),
                        name: name.clone(),
                        labels: Vec::new(),
                    },
                    redo_command: command,
                });
            }
            Command::SetWorklog { timestamp, task_id, date, resource_name, fraction } => {
                let resource_id = self.flow_state.resources.iter()
                    .find(|(_, res)| res.name == *resource_name)
                    .map(|(id, _)| *id);
                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }
                let resource_id = resource_id.unwrap();
                let current_worklog = self.flow_state.worklogs
                    .get(task_id)
                    .and_then(|resource_map| resource_map.get(&resource_id))
                    .and_then(|date_map| date_map.get(date))
                    .cloned();

                let undo_worklog = current_worklog.unwrap_or(Worklog {
                    task_id: *task_id,
                    date: *date,
                    resource_id: resource_id.clone(),
                    fraction: 0,
                });

                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::SetWorklog {
                        timestamp: timestamp.clone(),
                        task_id: *task_id,
                        date: *date,
                        resource_name: resource_name.clone(),
                        fraction: undo_worklog.fraction,
                    },
                    redo_command: command,
                });
            }
            Command::SetAbsence { timestamp, resource_name, start_date, days } => {
                let resource_id = self.flow_state.resources.iter()
                    .find(|(_, res)| res.name == *resource_name)
                    .map(|(id, _)| *id);
                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }
                let resource_id = resource_id.unwrap();

                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::SetAbsence {
                        timestamp: timestamp.clone(),
                        resource_name: resource_name.clone(),
                        start_date: start_date.clone(),
                        days: Duration { days: 0, fraction: 0 },
                    },
                    redo_command: command,
                });
            }
            Command::AddMilestone { timestamp, title, date } => {
                let milestone = Milestone {
                    date: date.clone(),
                    title: title.clone(),
                };
                self.flow_state.execute_command(command.clone())?;
                self.append_to_command_history(CommandRecord {
                    undo_command: Command::RemoveMilestone {
                        timestamp: timestamp.clone(),
                        title: title.clone(),
                    },
                    redo_command: command,
                });
            }
            Command::RemoveMilestone { timestamp, title } => {
                if let Some(milestone) = self.flow_state.milestones.iter()
                    .find(|m| m.title == *title)
                    .cloned() {
                    self.flow_state.execute_command(command.clone())?;
                    self.append_to_command_history(CommandRecord {
                        undo_command: Command::AddMilestone {
                            timestamp: timestamp.clone(),
                            title: milestone.title,
                            date: milestone.date,
                        },
                        redo_command: command,
                    });
                } else {
                    return Err(format!("No milestone found with the title '{}'", title));
                }
            }
            _ => return Err("Command not implemented".to_string()),
        }
        Ok(())
    }

    fn undo(&mut self) -> Result<(), String> {
        if self.num_commands_applied == 0 {
            return Err("No commands to undo".to_string());
        }
        let command_record = &self.command_stack[self.num_commands_applied - 1];
        self.flow_state.execute_command(command_record.undo_command.clone())?;
        self.num_commands_applied -= 1;
        Ok(())
    }

    fn redo(&mut self) -> Result<(), String> {
        if self.num_commands_applied >= self.command_stack.len() {
            return Err("No commands to redo".to_string());
        }
        let command_record = &self.command_stack[self.num_commands_applied];
        self.flow_state.execute_command(command_record.redo_command.clone())?;
        self.num_commands_applied += 1;
        Ok(())
    }

    fn append_to_command_history(&mut self, command_record: CommandRecord) {
        if self.num_commands_applied < self.command_stack.len() {
            self.command_stack.truncate(self.num_commands_applied);
        }
        self.command_stack.push(command_record);
        self.num_commands_applied = self.command_stack.len();
    }

    fn next_task_id(&mut self) -> TaskId {
        let id = self.flow_state.next_task_id;
        self.flow_state.next_task_id += 1;
        id
    }
}

impl FlowState {
    fn execute_command(&mut self, command: Command) -> Result<(), String> {
        match command {
            Command::CreateTeam { name, .. } => {
                if self.teams.values().any(|team| team.name == name) {
                    return Err(format!("A team with the name '{}' already exists", name));
                }

                let team_id = self.next_team_id();
                self.teams.insert(team_id, Team::new(name.clone()));
            }
            Command::RenameTeam { old_name, new_name, .. } => {
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
                    team.name = new_name;
                }
            }
            Command::DeleteTeam { name, .. } => {
                let team_id = self.teams.iter()
                    .find(|(_, team)| team.name == name)
                    .map(|(id, _)| *id);

                if let Some(team_id) = team_id {
                    self.teams.remove(&team_id);
                } else {
                    return Err(format!("No team found with the name '{}'", name));
                }
            }
            Command::CreateResource { name, team_name, .. } => {
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
            }
            Command::RenameResource { old_name, new_name, .. } => {
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
                    resource.name = new_name;
                }
            }
            Command::SwitchTeam { resource_name, new_team_name, .. } => {
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
                    if let Some(current_team) = self.teams.get_mut(&current_team_id) {
                        current_team.resources.remove(&resource_id);
                    }
                }

                if let Some(new_team) = self.teams.get_mut(&new_team_id) {
                    new_team.resources.insert(resource_id);
                }

                if let Some(resource) = self.resources.get_mut(&resource_id) {
                    resource.team_id = new_team_id;
                }
            }
            Command::DeleteResource { name, .. } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == name)
                    .map(|(id, _)| *id);

                if let Some(resource_id) = resource_id {
                    if let Some(resource) = self.resources.remove(&resource_id) {
                        if let Some(team) = self.teams.get_mut(&resource.team_id) {
                            team.resources.remove(&resource_id);
                        }
                    }
                } else {
                    return Err(format!("No resource found with the name '{}'", name));
                }
            },
            Command::CreateTask { timestamp, id, ticket, title, duration, .. } => {
                let task = Task::new(timestamp, id, ticket, title, duration);
                self.tasks.insert(id, task);
                self.unassigned_tasks.insert(id);
            }
            Command::UpdateTask { id, ticket, title, duration, .. } => {
                if let Some(task) = self.tasks.get_mut(&id) {
                    task.ticket = ticket;
                    task.title = title;
                    task.duration = duration;
                } else {
                    return Err(format!("Task with id {} not found", id));
                }
            }
            Command::DeleteTask { id, .. } => {
                if let Some(task) = self.tasks.remove(&id) {
                    if let Some(resource) = self.resources.get_mut(&task.assignee.unwrap()) {
                        resource.assigned_tasks.remove(&id);
                    }
                } else {
                    return Err(format!("Task with id {} not found", id));
                }
            }
            Command::AssignTask { task_id, resource_name, .. } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == resource_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }

                let resource_id = resource_id.unwrap();

                if let Some(task) = self.tasks.get_mut(&task_id) {
                    task.assignee = Some(resource_id);
                    if let Some(resource) = self.resources.get_mut(&resource_id) {
                        resource.assigned_tasks.insert(task_id);
                    }
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::UnassignTask { task_id, .. } => {
                if let Some(task) = self.tasks.get_mut(&task_id) {
                    if let Some(resource_id) = task.assignee {
                        if let Some(resource) = self.resources.get_mut(&resource_id) {
                            resource.assigned_tasks.remove(&task_id);
                        }
                    }
                    task.assignee = None;
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::AddWatcher { task_id, resource_name, .. } => {
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
                        resource.watched_tasks.insert(task_id);
                    }
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::RemoveWatcher { task_id, resource_name, .. } => {
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
                        resource.watched_tasks.remove(&task_id);
                    }
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            Command::CreateLabel { name, .. } => {
                if self.labels.values().any(|label| label.name == name) {
                    return Err(format!("A label with the name '{}' already exists", name));
                }

                let label_id = self.next_label_id();
                self.labels.insert(label_id, Label { name: name.clone() });
            }
            Command::RenameLabel { old_name, new_name, .. } => {
                let label_id = self.labels.iter()
                    .find(|(_, label)| label.name == old_name)
                    .map(|(id, _)| *id);
                if label_id.is_none() {
                    return Err(format!("No label found with the name '{}'", old_name));
                }
                let label_id = label_id.unwrap();
                self.labels.insert(label_id, Label { name: new_name.clone() });
            }
            Command::DeleteLabel { name, .. } => {
                let label_id = self.labels.iter()
                    .find(|(_, label)| label.name == name)
                    .map(|(id, _)| *id);
                if let Some(label_id) = label_id {
                    self.labels.remove(&label_id);
                } else {
                    return Err(format!("No label found with the name '{}'", name));
                }
            }
            Command::CreateFilter { name, labels, .. } => {
                if self.filters.values().any(|filter| filter.name == name) {
                    return Err(format!("A filter with the name '{}' already exists", name));
                }

                let label_ids: BTreeSet<LabelId> = labels.iter()
                    .filter_map(|label_name| {
                        self.get_label_id(label_name).or_else(|| {
                            eprintln!("Warning: Label '{}' not found", label_name);
                            None
                        })
                    })
                    .collect();
                let filter_id = self.next_filter_id();
                self.filters.insert(filter_id, Filter { name: name.clone(), labels: label_ids });
            }
            Command::RenameFilter { old_name, new_name, .. } => {
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
                    filter.name = new_name;
                }
            }
            Command::DeleteFilter { name, .. } => {
                let filter_id = self.filters.iter()
                    .find(|(_, filter)| filter.name == name)
                    .map(|(id, _)| *id);
                if let Some(filter_id) = filter_id {
                    self.filters.remove(&filter_id);
                } else {
                    return Err(format!("No filter found with the name '{}'", name));
                }
            }
            Command::SetWorklog { task_id, date, resource_name, fraction, .. } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == resource_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }

                let resource_id = resource_id.unwrap();

                if fraction == 0 {
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
                    let worklog = Worklog {
                        task_id,
                        date,
                        resource_id,
                        fraction,
                    };

                    self.worklogs.entry(task_id)
                        .or_default()
                        .entry(resource_id)
                        .or_default()
                        .insert(date, worklog);
                }
            }
            Command::CompoundCommand { timestamp, commands } => {
                let mut flow_state_clone = self.clone();
                for cmd in commands {
                    flow_state_clone.execute_command(cmd.clone())?;
                }
                *self = flow_state_clone;
            }
            Command::SetAbsence { resource_name, start_date, days, .. } => {
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

                self.resources.get_mut(&resource_id)
                    .ok_or_else(|| format!("Resource with id {} not found", resource_id))?
                    .absences.push(absence);

                todo!("Implement updating the absence")
            }
            Command::AddMilestone { timestamp, title, date } => {
                let milestone = Milestone {
                    title,
                    date,
                };
                self.milestones.push(milestone.clone());
                self.date_to_milestones
                    .entry(date)
                    .or_insert_with(Vec::new)
                    .push(milestone);
            }
            Command::RemoveMilestone { timestamp, title } => {
                if let Some(pos) = self.milestones.iter().position(|m| m.title == title) {
                    let milestone = self.milestones.remove(pos);
                    if let Some(milestones) = self.date_to_milestones.get_mut(&milestone.date) {
                        milestones.retain(|m| m.title != title);
                        if milestones.is_empty() {
                            self.date_to_milestones.remove(&milestone.date);
                        }
                    }
                } else {
                    return Err(format!("No milestone found with the title '{}'", title));
                }
            }
            _ => return Err("Command not implemented".to_string()),
        }
        Ok(())
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

    #[test]
    fn test_undo_redo_create_task() {
        let mut app = Application::new();
        
        let timestamp = Utc::now();
        let task_id = app.next_task_id();
        let ticket = "TASK-123".to_string();
        let title = "Implement feature X".to_string();
        let duration = Duration { days: 2, fraction: 50 };

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
}