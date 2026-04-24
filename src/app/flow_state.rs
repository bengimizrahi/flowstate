use crate::app::*;
use chrono::{NaiveDate, DateTime, Utc};
use std::collections::{BTreeMap, HashMap, BTreeSet};

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
    pub fn new() -> Self {
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
        let date: NaiveDate = NaiveDate::from_ymd_opt(1970,1,1).unwrap();
        flow_state.rebuild_cache(date);
        flow_state
    }

    pub fn from_commands(commands: &Vec<Command>, date: NaiveDate) -> Result<Self, String> {
        let mut flow_state = FlowState::new();
        for command in commands {
            flow_state.execute_command_and_generate_inverse(command.clone())?;
        }
        flow_state.rebuild_cache(date);
        flow_state.reset_ids();
        Ok(flow_state)
    }

    pub fn execute_command_and_generate_inverse(&mut self, command: Command) -> Result<Command, String> {
        let timestamp = command.timestamp;
        match command.details {
            CommandDetails::Void => Ok(Command{timestamp, details: CommandDetails::Void}),
            CommandDetails::CreateTeam { name} => {
                if self.teams.values().any(|team| team.name == name) {
                    return Err(format!("A team with the name '{}' already exists", name));
                }

                let team_id = self.next_team_id();
                self.teams.insert(team_id, Team::new(timestamp, name.clone()));
                Ok(Command {timestamp, details: CommandDetails::DeleteTeam { name }})
            }
            CommandDetails::RenameTeam { old_name, new_name } => {
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
                Ok(Command {timestamp, details: CommandDetails::RenameTeam { old_name: new_name, new_name: old_name }})
            }
            CommandDetails::DeleteTeam { name } => {
                let team_id = self.teams.iter()
                    .find(|(_, team)| team.name == name)
                    .map(|(id, _)| *id);

                if let Some(team_id) = team_id {
                    self.teams.remove(&team_id);
                } else {
                    return Err(format!("No team found with the name '{}'", name));
                }
                Ok(Command { timestamp, details: CommandDetails::CreateTeam { name } })
            }
            CommandDetails::CreateResource { name, team_name } => {
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
                self.resources.insert(resource_id, Resource::new(timestamp, name.clone(), team_id.unwrap()));

                if let Some(team) = self.teams.get_mut(&team_id.unwrap()) {
                    team.resources.insert(resource_id);
                }
                Ok(Command { timestamp, details: CommandDetails::DeleteResource { name } })
            }
            CommandDetails::RenameResource { old_name, new_name } => {
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
                Ok(Command { timestamp, details: CommandDetails::RenameResource { old_name: new_name, new_name: old_name } })
            }
            CommandDetails::SwitchTeam { resource_name, new_team_name } => {
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
                    Ok(Command { timestamp, details: CommandDetails::SwitchTeam { resource_name, new_team_name: old_team_name } })
                } else {
                    Err("Resource has no current team".to_string())
                }
            }
            CommandDetails::DeleteResource { name } => {
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
                                    Command { timestamp, details: CommandDetails::CreateResource {
                                        name: resource.name,
                                        team_name: team.name.clone()
                                    }}
                                })
                        })
                        .ok_or_else(|| format!("Failed to create resource '{}'", name))
                } else {
                    return Err(format!("No resource found with the name '{}'", name));
                }
            },
            CommandDetails::CreateTask { id, ticket, title, duration } => {
                let task = Task::new(timestamp, id, ticket, title, duration);
                self.tasks.insert(id, task);
                Ok(Command { timestamp, details: CommandDetails::DeleteTask { id } })
            }
            CommandDetails::UpdateTask { id, ticket, title, duration } => {
                if let Some(task) = self.tasks.get_mut(&id) {
                    let original_ticket = task.ticket.clone();
                    let original_title = task.title.clone();
                    let original_duration = task.duration.clone();

                    task.ticket = ticket;
                    task.title = title;
                    task.duration = duration;

                    Ok(Command { timestamp, details: CommandDetails::UpdateTask {
                        id,
                        ticket: original_ticket,
                        title: original_title,
                        duration: original_duration,
                    }})
                } else {
                    return Err(format!("Task with id {} not found", id));
                }
            }
            CommandDetails::DeleteTask { id } => {
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
                    Ok(Command { timestamp, details: CommandDetails::CreateTask {
                        id,
                        ticket,
                        title,
                        duration,
                    }})
                } else {
                    return Err(format!("Task with id {} not found", id));
                }
            }
            CommandDetails::PrioritizeTask { task_id, to_top } => {
                if let Some(task) = self.tasks.get(&task_id) {
                    if let Some(assignee_id) = task.assignee {
                        if let Some(resource) = self.resources.get_mut(&assignee_id) {
                            let pos = resource.assigned_tasks.iter().position(|&id| id == task_id);
                            if let Some(pos) = pos {
                                if to_top {
                                    self.execute_command_and_generate_inverse(Command { timestamp, details: CommandDetails::ChangeTaskPriority { task_id, delta: (-(pos as i32)) } })
                                } else {
                                    self.execute_command_and_generate_inverse(Command { timestamp, details: CommandDetails::ChangeTaskPriority { task_id, delta: if pos > 0 {-1} else {0} } })
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
            CommandDetails::DeprioritizeTask { task_id, to_bottom} => {
                if let Some(task) = self.tasks.get(&task_id) {
                    if let Some(assignee_id) = task.assignee {
                        if let Some(resource) = self.resources.get_mut(&assignee_id) {
                            let pos = resource.assigned_tasks.iter().position(|&id| id == task_id);
                            if let Some(pos) = pos {
                                if to_bottom {
                                    let delta = (resource.assigned_tasks.len() - 1 - pos) as i32;
                                    self.execute_command_and_generate_inverse(Command { timestamp, details: CommandDetails::ChangeTaskPriority { task_id, delta } })
                                } else if pos < resource.assigned_tasks.len() - 1 {
                                    self.execute_command_and_generate_inverse(Command { timestamp, details: CommandDetails::ChangeTaskPriority { task_id, delta: 1 } })
                                } else {
                                    self.execute_command_and_generate_inverse(Command { timestamp, details: CommandDetails::ChangeTaskPriority { task_id, delta: 0 } })
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
            CommandDetails::ChangeTaskPriority { task_id, delta } => {
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
                                Ok(Command { timestamp, details: CommandDetails::ChangeTaskPriority { task_id, delta: -delta } })
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
            CommandDetails::AssignTask { task_id, resource_name } => {
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
                            Ok(Command { timestamp, details: CommandDetails::AssignTask { task_id, resource_name: old_assignee_name } })
                        }
                        None => {
                            task.assignee = Some(resource_id);
                            if let Some(resource) = self.resources.get_mut(&resource_id) {
                                resource.assigned_tasks.insert(0, task_id);
                            }
                            Ok(Command { timestamp, details: CommandDetails::UnassignTask { task_id } })
                        }
                    }
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            CommandDetails::UnassignTask { task_id } => {
                if let Some(task) = self.tasks.get_mut(&task_id) {
                    if let Some(old_assignee_id) = task.assignee.clone() {
                        if let Some(old_assignee_name) = self.resources.get(&old_assignee_id)
                            .map(|res| res.name.clone()) {
                                task.assignee = None;
                                if let Some(resource) = self.resources.get_mut(&old_assignee_id) {
                                    resource.assigned_tasks.retain(|&x| x != task_id);
                                }
                                Ok(Command { timestamp, details: CommandDetails::AssignTask { task_id, resource_name: old_assignee_name } })
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
            CommandDetails::AddWatcher { task_id, resource_name } => {
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
                    Ok(Command { timestamp, details: CommandDetails::RemoveWatcher { task_id, resource_name } })
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            CommandDetails::RemoveWatcher { task_id, resource_name } => {
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
                    Ok(Command { timestamp, details: CommandDetails::AddWatcher { task_id, resource_name } })
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            CommandDetails::CreateLabel { name } => {
                if self.labels.values().any(|label| label.name == name) {
                    return Err(format!("A label with the name '{}' already exists", name));
                }

                let label_id = self.next_label_id();
                self.labels.insert(label_id, Label { name: name.clone() });

                Ok(Command { timestamp, details: CommandDetails::DeleteLabel { name } })
            }
            CommandDetails::RenameLabel { old_name, new_name } => {
                let label_id = self.labels.iter()
                    .find(|(_, label)| label.name == old_name)
                    .map(|(id, _)| *id);
                if label_id.is_none() {
                    return Err(format!("No label found with the name '{}'", old_name));
                }
                let label_id = label_id.unwrap();
                self.labels.insert(label_id, Label { name: new_name.clone() });
                Ok(Command { timestamp, details: CommandDetails::RenameLabel { new_name, old_name } })
            }

            CommandDetails::DeleteLabel { name } => {
                let label_id = self.labels.iter()
                    .find(|(_, label)| label.name == name)
                    .map(|(id, _)| *id);
                if let Some(label_id) = label_id {
                    self.labels.remove(&label_id);
                    Ok(Command { timestamp, details: CommandDetails::CreateLabel { name } })
                } else {
                    return Err(format!("No label found with the name '{}'", name));
                }
            }
            CommandDetails::AddLabelToTask { task_id, label_name } => {
                let label_id = self.get_label_id(&label_name);
                if label_id.is_none() {
                    return Err(format!("No label found with the name '{}'", label_name));
                }
                let label_id = label_id.unwrap();

                if let Some(task) = self.tasks.get_mut(&task_id) {
                    task.label_ids.insert(label_id);
                    Ok(Command { timestamp, details: CommandDetails::RemoveLabelFromTask { task_id, label_name } })
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            CommandDetails::RemoveLabelFromTask { task_id, label_name } => {
                let label_id = self.get_label_id(&label_name);
                if label_id.is_none() {
                    return Err(format!("No label found with the name '{}'", label_name));
                }
                let label_id = label_id.unwrap();

                if let Some(task) = self.tasks.get_mut(&task_id) {
                    task.label_ids.remove(&label_id);
                    Ok(Command { timestamp, details: CommandDetails::AddLabelToTask { task_id, label_name } })
                } else {
                    return Err(format!("Task with id {} not found", task_id));
                }
            }
            CommandDetails::CreateModifyFilter { name, labels, is_favorite } => {
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
                    Ok(Command { timestamp, details: CommandDetails::CreateModifyFilter { name, labels: old_label_names, is_favorite: old_is_favorite } })
                } else {
                    let filter_id = self.next_filter_id();
                    self.filters.insert(filter_id, Filter { name: name.clone(), labels: label_ids, is_favorite: false });
                    Ok(Command { timestamp, details: CommandDetails::DeleteFilter { name } })
                }
            }
            CommandDetails::RenameFilter { old_name, new_name } => {
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
                Ok(Command { timestamp, details: CommandDetails::RenameFilter { old_name: new_name, new_name: old_name } })
            }
            CommandDetails::DeleteFilter { name } => {
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
                    Ok(Command { timestamp, details: CommandDetails::CreateModifyFilter { name, labels, is_favorite: filter.is_favorite } })
                } else {
                    return Err(format!("No filter found with the name '{}'", name));
                }
            }
            CommandDetails::SetWorklog { task_id, date, resource_name, fraction } => {
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
                    create_timestamp: DateTime::from_naive_utc_and_offset(date.and_hms_opt(0, 0, 0).unwrap(), Utc),
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

                Ok(Command { timestamp, details: CommandDetails::SetWorklog {
                    task_id,
                    date,
                    resource_name,
                    fraction: previous_fraction,
                }})
            }
            CommandDetails::SetAbsence { resource_name, start_date, days } => {
                let resource_id = self.resources.iter()
                    .find(|(_, res)| res.name == resource_name)
                    .map(|(id, _)| *id);

                if resource_id.is_none() {
                    return Err(format!("No resource found with the name '{}'", resource_name));
                }

                let resource_id = resource_id.unwrap();
                let absence = Absence {
                    create_timestamp: timestamp,
                    start_date,
                    duration: days,
                };
                if let Some(absences) = self.resources.get_mut(&resource_id).map(|r| &mut r.absences) {
                    absences.retain(|a| !a.intersects(&absence));
                    if days > TaskDuration::zero() {
                        absences.push(absence);
                    }
                }
                Ok(Command { timestamp, details: CommandDetails::SetAbsence {
                    resource_name,
                    start_date,
                    days: if days > TaskDuration::zero() { TaskDuration::zero() } else { days },
                }})
            }
            CommandDetails::AddMilestone { title, date } => {
                let milestone = Milestone {
                    title,
                    date,
                };
                self.milestones.push(milestone.clone());
                Ok(Command { timestamp, details: CommandDetails::RemoveMilestone { title: milestone.title } })
            }
            CommandDetails::RemoveMilestone { title } => {
                if let Some(pos) = self.milestones.iter().position(|m| m.title == title) {
                    let milestone = self.milestones.remove(pos);
                    Ok(Command { timestamp, details: CommandDetails::AddMilestone { title: milestone.title, date: milestone.date } })
                } else {
                    return Err(format!("No milestone found with the title '{}'", title));
                }
            }
            CommandDetails::CompoundCommand { commands } => {
                let mut flow_state_clone = self.clone();
                let mut undo_commands = Vec::new();
                for cmd in commands {
                    let undo_cmd = flow_state_clone.execute_command_and_generate_inverse(cmd)?;
                    undo_commands.push(undo_cmd);
                }
                undo_commands.reverse();
                *self = flow_state_clone;
                Ok(Command { timestamp, details: CommandDetails::CompoundCommand { commands: undo_commands }})
            }
        }
    }

    pub fn execute_command_generate_inverse_and_rebuild_cache(&mut self, command: Command, date: NaiveDate) -> Result<Command, String> {
        let undo_command = self.execute_command_and_generate_inverse(command)?;
        self.rebuild_cache(date);
        Ok(undo_command)
    }

    pub fn rebuild_cache(&mut self, date: NaiveDate) {
        self.flow_state_cache = FlowStateCache::from(self, date);
    }

    fn reset_ids(&mut self) {
        self.next_team_id = self.teams.keys().max().map_or(1, |max_id| max_id + 1);
        self.next_resource_id = self.resources.keys().max().map_or(1, |max_id| max_id + 1);
        self.next_task_id = self.tasks.keys().max().map_or(1, |max_id| max_id + 1);
        self.next_label_id = self.labels.keys().max().map_or(1, |max_id| max_id + 1);
        self.next_filter_id = self.filters.keys().max().map_or(1, |max_id| max_id + 1);
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