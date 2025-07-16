use std::collections::HashMap;
use chrono::{Local, Duration};
use super::{types::*, commands::*, cache::Level1Cache};

#[derive(Debug, Clone)]
pub struct FlowState {
    pub(crate) command_history: Vec<CommandRecord>,
    command_count: usize,
    
    pub(crate) teams: HashMap<TeamName, TeamName>,
    resources: HashMap<ResourceName, Resource>,

    level1_cache: Level1Cache,
}

impl FlowState {
    pub fn new() -> Self {
        let mut flow_state = FlowState {
            command_history: Vec::new(),
            command_count: 0,

            teams: HashMap::new(),
            resources: HashMap::new(),

            level1_cache: Level1Cache::new(),
        };
        flow_state.build_cache();
        flow_state
    }

    pub fn l1(&self) -> &Level1Cache {
        &self.level1_cache
    }

    fn build_cache(&mut self) {
        self.level1_cache.start_date = Local::now().date_naive() - Duration::days(30);
        self.level1_cache.end_date = Local::now().date_naive() + Duration::days(30);
    }

    // Team operations
    pub fn create_team(&mut self, team_name: TeamName) -> Result<(), String> {
        let team_name_clone = team_name.clone();
        let team_name_for_undo = team_name.clone();
        self.execute_command(&Command::CreateTeam(team_name))?;

        self.append_to_command_history(CommandRecord {
            undo_command: Command::DeleteTeam(team_name_clone),
            redo_command: Command::CreateTeam(team_name_for_undo),
        });
        
        Ok(())
    }

    pub fn rename_team(&mut self, old_name: &str, new_name: &str) -> Result<(), String> {
        self.execute_command(&Command::RenameTeam(old_name.to_string(), new_name.to_string()))?;

        self.append_to_command_history(CommandRecord {
            undo_command: Command::RenameTeam(new_name.to_string(), old_name.to_string()),
            redo_command: Command::RenameTeam(old_name.to_string(), new_name.to_string()),
        });

        Ok(())
    }

    pub fn delete_team(&mut self, team_name: &str) -> Result<(), String> {
        let team = self.teams.remove(team_name)
            .ok_or_else(|| format!("Team '{team_name:?}' does not exist"))?;

        self.append_to_command_history(CommandRecord {
            undo_command: Command::CreateTeam(team),
            redo_command: Command::DeleteTeam(team_name.to_string()),
        });

        Ok(())
    }

    // Resource operations
    pub fn create_resource(&mut self, resource: Resource) -> Result<(), String> {
        let resource_name = resource.name.clone();
        let resource_for_undo = resource.clone();
        self.execute_command(&Command::CreateResource(resource))?;

        self.append_to_command_history(CommandRecord {
            undo_command: Command::DeleteResource(resource_name),
            redo_command: Command::CreateResource(resource_for_undo),
        });
        
        Ok(())
    }

    pub fn rename_resource(&mut self, old_name: &str, new_name: &str) -> Result<(), String> {
        self.execute_command(&Command::RenameResource(old_name.to_string(), new_name.to_string()))?;

        self.append_to_command_history(CommandRecord {
            undo_command: Command::RenameResource(new_name.to_string(), old_name.to_string()),
            redo_command: Command::RenameResource(old_name.to_string(), new_name.to_string()),
        });

        // TODO: Update all references to the old resource name
        Ok(())
    }

    pub fn delete_resource(&mut self, resource_name: &str) -> Result<(), String> {
        let resource = self.resources.remove(resource_name)
            .ok_or_else(|| format!("Resource '{resource_name:?}' does not exist"))?;

        // TODO: Check if resource is used in tasks before deletion

        self.append_to_command_history(CommandRecord {
            undo_command: Command::CreateResource(resource),
            redo_command: Command::DeleteResource(resource_name.to_string()),
        });

        Ok(())
    }

    // Command operations
    fn append_to_command_history(&mut self, command_record: CommandRecord) {
        if self.command_count < self.command_history.len() {
            self.command_history.truncate(self.command_count);
        }
        self.command_history.push(command_record);
        self.command_count = self.command_history.len();
    }

    pub fn invoke(&mut self, command: &Command) -> Result<(), String> {
        match command {
            Command::CreateTeam(team) => {
                self.create_team(team.clone())
            }
            Command::DeleteTeam(team_name) => {
                self.delete_team(team_name)
            }
            Command::RenameTeam(old_name, new_name) => {
                self.rename_team(old_name, new_name)
            }
            Command::CreateResource(resource) => {
                self.create_resource(resource.clone())
            }
            Command::RenameResource(old_name, new_name) => {
                self.rename_resource(old_name, new_name)
            }
            Command::DeleteResource(resource_name) => {
                self.delete_resource(resource_name)
            }
            Command::CompoundCommand(_) => {
                Err("Compound commands are not supported directly".to_string())
            }
        }
    }

    pub fn undo(&mut self) -> Result<(), String> {
        if self.command_count == 0 {
            return Err("No more commands to undo".to_string());
        }

        self.command_count -= 1;
        let command = self.command_history[self.command_count].undo_command.clone();
        self.execute_command(&command)
            .map_err(|e| format!("Undo failed: {}", e))
    }

    pub fn redo(&mut self) -> Result<(), String> {
        if self.command_count >= self.command_history.len() {
            return Err("No more commands to redo".to_string());
        }

        let command = self.command_history[self.command_count].redo_command.clone();
        self.execute_command(&command)
            .map_err(|e| format!("Redo failed: {}", e))?;

        self.command_count += 1;
        Ok(())
    }

    fn execute_command(&mut self, command: &Command) -> Result<(), String> {
        match command {
            Command::CreateTeam(team_name) => {
                if self.teams.contains_key(team_name) {
                    return Err(format!("Team '{}' already exists", team_name));
                }
                self.teams.insert(team_name.clone(), team_name.clone());
                Ok(())
            }
            Command::DeleteTeam(team_name) => {
                // TODO: If the team has a resource, don't allow deletion

                self.teams.remove(team_name)
                    .map(|_| ())
                    .ok_or_else(|| format!("Team '{}' does not exist", team_name))
            }
            Command::RenameTeam(old_name, new_name) => {
                if !self.teams.contains_key(old_name) {
                    return Err(format!("Team '{}' does not exist", old_name));
                }
                if self.teams.contains_key(new_name) {
                    return Err(format!("Team '{}' already exists", new_name));
                }

                // TODO: Update all references to the old team name

                let _team = self.teams.remove(old_name).unwrap();
                let renamed_team = new_name.clone();
                self.teams.insert(new_name.clone(), renamed_team);
                Ok(())
            }

            Command::CreateResource(resource) => {
                if self.resources.contains_key(&resource.name) {
                    return Err(format!("Resource '{}' already exists", resource.name));
                }
                self.resources.insert(resource.name.clone(), resource.clone());
                Ok(())
            }
            Command::RenameResource(old_name, new_name) => {
                if !self.resources.contains_key(old_name) {
                    return Err(format!("Resource '{}' does not exist", old_name));
                }
                if self.resources.contains_key(new_name) {
                    return Err(format!("Resource '{}' already exists", new_name));
                }

                // TODO: Update all references to the old resource name

                let resource = self.resources.remove(old_name).unwrap();
                let renamed_resource = Resource {
                    name: new_name.clone(),
                    .. resource.clone()
                };
                self.resources.insert(new_name.clone(), renamed_resource);
                Ok(())

                // TODO: Add functionality to change resource team
            }
            Command::DeleteResource(resource_name) => {
                // TODO: Check if resource is used in tasks before deletion

                self.resources.remove(resource_name)
                    .map(|_| ())
                    .ok_or_else(|| format!("Resource '{}' does not exist", resource_name))
            }
            Command::CompoundCommand(commands) => {
                let initial_state = self.clone();
                for cmd in commands {
                    if let Err(err) = self.execute_command(cmd) {
                        *self = initial_state;
                        return Err(format!("Compound command failed executing: {:?} (Err: {:?}", cmd, err));
                    }
                }
                Ok(())
            }
        }
    }
}
