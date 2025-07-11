use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type TeamName = String;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Team {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    CreateTeam(Team),
    RenameTeam(TeamName, TeamName),
    DeleteTeam(TeamName),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandRecord {
    pub undo_command: Command,
    pub redo_command: Command,
}

#[derive(Debug)]
pub struct FlowState {
    teams: HashMap<TeamName, Team>,

    command_history: Vec<CommandRecord>,
    command_count: usize,
}

impl FlowState {
    pub fn new() -> Self {
        FlowState {
            teams: HashMap::new(),
            
            command_history: Vec::new(),
            command_count: 0,
        }
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

    fn append_to_command_history(&mut self, command_record: CommandRecord) {
        if self.command_count < self.command_history.len() {
            self.command_history.truncate(self.command_count);
        }
        self.command_history.push(command_record);
        self.command_count = self.command_history.len();
    }

    pub fn create_team(&mut self, team: Team) -> Result<(), String> {
        let team_name = team.name.clone();
        let team_for_undo = team.clone();
        self.execute_command(&Command::CreateTeam(team))?;

        self.append_to_command_history(CommandRecord {
            undo_command: Command::DeleteTeam(team_name),
            redo_command: Command::CreateTeam(team_for_undo),
        });
        
        Ok(())
    }

    pub fn rename_team(&mut self, old_name: &str, new_name: &str) -> Result<(), String> {
        self.execute_command(&Command::RenameTeam(old_name.to_string(), new_name.to_string()))?;

        self.append_to_command_history(CommandRecord {
            undo_command: Command::RenameTeam(new_name.to_string(), old_name.to_string()),
            redo_command: Command::RenameTeam(old_name.to_string(), new_name.to_string()),
        });

        todo!("Wherever old team name is used, it should be replaced with the new name");

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
            Command::CreateTeam(team) => {
                if self.teams.contains_key(&team.name) {
                    return Err(format!("Team '{}' already exists", team.name));
                }
                self.teams.insert(team.name.clone(), team.clone());
                Ok(())
            }
            Command::DeleteTeam(team_name) => {
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

                let team = self.teams.remove(old_name).unwrap();
                let renamed_team = Team { name: new_name.clone() };
                self.teams.insert(new_name.clone(), renamed_team);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_delete_team() {
        let mut flow_state = FlowState::new();
        
        let team = Team { name: "Test Team".to_string() };
        let team_name: String = team.name.clone();

        assert!(flow_state.create_team(team).is_ok());
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

        let team = Team { name: "Test Team".to_string() };
        let team_name = team.name.clone();
        
        assert!(flow_state.create_team(team.clone()).is_ok());
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
        let team = Team { name: "Test Team".to_string() };
        let team_name = team.name.clone();
        assert!(flow_state.create_team(team).is_ok());

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
        let team = Team { name: "Test Team".to_string() };
        let team_name = team.name.clone();
        assert!(flow_state.create_team(team).is_ok());
        assert_eq!(flow_state.teams.len(), 1);
        assert!(flow_state.teams.contains_key(&team_name));
        assert!(flow_state.save_as_yaml().is_ok());
        println!("After save: {flow_state:#?}");

        let mut loaded_flow_state = FlowState::new();
        assert!(loaded_flow_state.load_from_yaml().is_ok());
        assert_eq!(loaded_flow_state.teams.len(), 1);
        assert!(loaded_flow_state.teams.contains_key(&team_name));
        assert_eq!(loaded_flow_state.teams.get(&team_name).unwrap().name, team_name);
        println!("After load: {loaded_flow_state:#?}");
    }
}