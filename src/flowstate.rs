use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type TeamName = String;
pub type ResourceName = String;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Resource {
    pub name: ResourceName,
    pub team_name: TeamName,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    CreateTeam(TeamName),
    RenameTeam(TeamName, TeamName),
    DeleteTeam(TeamName),

    CreateResource(Resource),
    RenameResource(ResourceName, ResourceName),
    DeleteResource(ResourceName),

    CompoundCommand(Vec<Command>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRecord {
    pub undo_command: Command,
    pub redo_command: Command,
}

#[derive(Debug, Clone)]
pub struct Level1Cache {
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

impl Level1Cache {
    pub fn new() -> Self {
        let current_date = chrono::Local::now().date_naive();
        Level1Cache {
            start_date: current_date,
            end_date: current_date,
        }
    }

    pub fn day(&self, offset: usize) -> chrono::NaiveDate {
        self.start_date + chrono::Duration::days(offset as i64)
    }

    pub fn num_days(&self) -> usize {
        self.end_date.signed_duration_since(self.start_date).num_days() as usize
    }
}
#[derive(Debug, Clone)]
pub struct FlowState {
    command_history: Vec<CommandRecord>,
    command_count: usize,
    
    teams: HashMap<TeamName, TeamName>,
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

    pub fn save_as_yaml(&self) -> Result<(), String> {
        let commands_to_serialize: Vec<Command> = self.command_history
            .iter()
            .map(|record| record.redo_command.clone())
            .collect();
        let commands_as_yaml = serde_yaml::to_string(&commands_to_serialize)
            .map_err(|e| e.to_string())?;
        std::fs::write("database.yaml", commands_as_yaml)
            .map_err(|e| e.to_string())
    }

    pub fn load_from_yaml(&mut self) -> Result<(), String> {
        let yaml_content = std::fs::read_to_string("database.yaml")
            .map_err(|e| e.to_string())?;
        let commands: Vec<Command> = serde_yaml::from_str(&yaml_content)
            .map_err(|e| e.to_string())?;

        let mut new_flow_state = FlowState::new();
        commands.iter().try_for_each(|command| new_flow_state.invoke(command))?;
        *self = new_flow_state;
        Ok(())
    }
}

impl FlowState {
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

        // todo!("Wherever old resource name is used, it should be replaced with the new name");

        Ok(())
    }

    pub fn delete_resource(&mut self, resource_name: &str) -> Result<(), String> {
        let resource = self.resources.remove(resource_name)
            .ok_or_else(|| format!("Resource '{resource_name:?}' does not exist"))?;

        // todo!("If the resource is used in a task, don't allow deletion");

        self.append_to_command_history(CommandRecord {
            undo_command: Command::CreateResource(resource),
            redo_command: Command::DeleteResource(resource_name.to_string()),
        });

        Ok(())
    }
}

impl FlowState {
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
                // todo!("If the team has a resource, don't allow deletion");

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

                // todo!("Wherever old team name is used, it should be replaced with the new name");

                let team = self.teams.remove(old_name).unwrap();
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

                // todo!("Wherever old resource name is used, it should be replaced with the new name");

                let resource = self.resources.remove(old_name).unwrap();
                let renamed_resource = Resource {
                    name: new_name.clone(),
                    .. resource.clone()
                };
                self.resources.insert(new_name.clone(), renamed_resource);
                Ok(())

                // todo!("How about changing the team of a resource?");
            }
            Command::DeleteResource(resource_name) => {
                // todo!("If the resource is used in a task, don't allow deletion");

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

impl FlowState {
    pub fn l1(&self) -> &Level1Cache {
        &self.level1_cache
    }

    fn build_cache(&mut self) {
        self.level1_cache.start_date = chrono::Local::now().date_naive() - chrono::Duration::days(30);
        self.level1_cache.end_date = chrono::Local::now().date_naive() + chrono::Duration::days(30);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_delete_team() {
        let mut flow_state = FlowState::new();
        
        let team_name: String = "Test Team".to_string();

        assert!(flow_state.create_team(team_name.clone()).is_ok());
        assert_eq!(flow_state.teams.len(), 1);
        assert!(flow_state.teams.contains_key(&team_name));

        assert!(flow_state.delete_team(&team_name).is_ok());
        assert_eq!(flow_state.teams.len(), 0);
        assert!(!flow_state.teams.contains_key(&team_name));
    }

    #[test]
    fn test_create_delete_team_undo_undo_redo_redo() {
        /* create a team, then delete the team, then undo two
         * times, then redo two times, expecting no teams 
         */
        let mut flow_state = FlowState::new();

        let team_name = "Test Team".to_string();
        
        assert!(flow_state.create_team(team_name.clone()).is_ok());
        assert_eq!(flow_state.teams.len(), 1);
        assert!(flow_state.teams.contains_key(&team_name));
        println!("After create: {flow_state:#?}");

        assert!(flow_state.delete_team(&team_name).is_ok());
        assert_eq!(flow_state.teams.len(), 0);
        assert!(!flow_state.teams.contains_key(&team_name));
        println!("After delete: {flow_state:#?}");
        
        assert!(flow_state.undo().is_ok());
        assert_eq!(flow_state.teams.len(), 1);
        assert!(flow_state.teams.contains_key(&team_name));
        println!("After undo: {flow_state:#?}");

        assert!(flow_state.undo().is_ok());
        assert_eq!(flow_state.teams.len(), 0);
        assert!(!flow_state.teams.contains_key(&team_name));
        println!("After undo: {flow_state:#?}");

        assert!(flow_state.undo().is_err());
        println!("After undo: {flow_state:#?}");
        
        assert!(flow_state.redo().is_ok());
        assert_eq!(flow_state.teams.len(), 1);
        assert!(flow_state.teams.contains_key(&team_name));
        println!("After redo: {flow_state:#?}");

        assert!(flow_state.redo().is_ok());
        assert_eq!(flow_state.teams.len(), 0);
        assert!(!flow_state.teams.contains_key(&team_name));
        println!("After redo: {flow_state:#?}");

        assert!(flow_state.redo().is_err());
        println!("After redo: {flow_state:#?}");
    }

    #[test]
    fn test_rename_team_undo_redo() {
        let mut flow_state = FlowState::new();
        let team_name = "Test Team".to_string();
        assert!(flow_state.create_team(team_name.clone()).is_ok());

        let new_team_name = "Renamed Team".to_string();
        assert!(flow_state.rename_team(&team_name, &new_team_name).is_ok());
        assert_eq!(flow_state.teams.len(), 1);
        assert!(flow_state.teams.contains_key(&new_team_name));
        assert!(!flow_state.teams.contains_key(&team_name));
        println!("After rename: {flow_state:#?}");

        assert!(flow_state.undo().is_ok());
        assert_eq!(flow_state.teams.len(), 1);
        assert!(flow_state.teams.contains_key(&team_name));
        assert!(!flow_state.teams.contains_key(&new_team_name));
        println!("After undo: {flow_state:#?}");

        assert!(flow_state.redo().is_ok());
        assert_eq!(flow_state.teams.len(), 1);
        assert!(flow_state.teams.contains_key(&new_team_name));
        assert!(!flow_state.teams.contains_key(&team_name));
        println!("After redo: {flow_state:#?}");
    }

    #[test]
    fn test_save_load_yaml() {
        let mut flow_state = FlowState::new();
        let team_name = "Test Team".to_string();
        assert!(flow_state.create_team(team_name.clone()).is_ok());
        assert_eq!(flow_state.teams.len(), 1);
        assert!(flow_state.teams.contains_key(&team_name));
        assert!(flow_state.save_as_yaml().is_ok());
        println!("After save: {flow_state:#?}");

        let mut loaded_flow_state = FlowState::new();
        assert!(loaded_flow_state.load_from_yaml().is_ok());
        assert_eq!(loaded_flow_state.teams.len(), 1);
        assert!(loaded_flow_state.teams.contains_key(&team_name));
        assert_eq!(loaded_flow_state.teams.get(&team_name).unwrap(), &team_name);
        println!("After load: {loaded_flow_state:#?}");
    }
}