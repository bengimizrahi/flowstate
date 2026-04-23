use crate::app_next::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    #[serde(skip)]
    filename: Option<String>,
    pub command_stack: Vec<CommandRecord>,
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

    pub fn load_from_yaml(yaml_filename: &str, date: NaiveDate) -> Result<Self, String> {
        let mut file = File::open(yaml_filename).map_err(|e| format!("Failed to open YAML file: {}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| format!("Failed to read YAML file: {}", e))?;

        let (num_commands_applied, command_stack, ): (usize, Vec<CommandRecord>) =
            serde_yaml::from_str(&contents).map_err(|e| format!("Failed to deserialize YAML: {}", e))?;

        let flow_state = FlowState::from_commands(&command_stack.iter().take(num_commands_applied)
            .map(|record| record.redo_command.clone()).collect::<Vec<_>>(), date)
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

    pub fn invoke_command(&mut self, command: Command, date: NaiveDate) -> Result<(), String> {
        println!("Invoking command: {:?}", command);
        let undo_command = self.flow_state.execute_command_generate_inverse_and_rebuild_cache(command.clone(), date)?;
        self.append_to_command_history(CommandRecord {
            undo_command,
            redo_command: command,
        });
        self.save_to_yaml()?;
        Ok(())
    }

    pub fn undo(&mut self, date: NaiveDate) -> Result<(), String> {
        if self.num_commands_applied == 0 {
            return Err("No commands to undo".to_string());
        }
        let command_record = &self.command_stack[self.num_commands_applied - 1];
        println!("Command for undo: {:?}", command_record.undo_command);
        self.flow_state.execute_command_generate_inverse_and_rebuild_cache(command_record.undo_command.clone(), date)?;
        self.num_commands_applied -= 1;
        self.save_to_yaml()?;
        Ok(())
    }

    pub fn redo(&mut self, date: NaiveDate) -> Result<(), String> {
        if self.num_commands_applied >= self.command_stack.len() {
            return Err("No commands to redo".to_string());
        }
        let command_record = &self.command_stack[self.num_commands_applied];
        println!("Command for redo: {:?}", command_record.redo_command);
        self.flow_state.execute_command_generate_inverse_and_rebuild_cache(command_record.redo_command.clone(), date)?;
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

#[cfg(test)]
mod tests {
    use crate::app_next::*;
    use chrono::{NaiveDate, Utc};

    #[test]
    fn test_create_team() {
        let mut app = Project::new("test_project.yaml");
        let timestamp = Utc::now();
        let team_name = "Development".to_string();

        let result = app.invoke_command(Command { timestamp, details: CommandDetails::CreateTeam { name: team_name } }, timestamp.date_naive());

        assert!(result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == "Development"));
    }

    #[test]
    fn test_undo_create_team() {
        let mut app = Project::new("test_project.yaml");
        let timestamp = Utc::now();
        let team_name = "Development".to_string();

        let result = app.invoke_command(Command { timestamp, details: CommandDetails::CreateTeam { name: team_name } }, timestamp.date_naive());
        assert!(result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == "Development"));

        let undo_result = app.undo(timestamp.date_naive());
        assert!(undo_result.is_ok());
        assert!(!app.flow_state.teams.values().any(|team| team.name == "Development"));
    }

    #[test]
    fn test_undo_redo_create_team() {
        let mut app = Project::new("test_project.yaml");
        let timestamp = Utc::now();
        let team_name = "Development".to_string();

        let result = app.invoke_command(Command { timestamp, details: CommandDetails::CreateTeam { name: team_name } }, timestamp.date_naive());
        assert!(result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == "Development"));

        let undo_result = app.undo(timestamp.date_naive());
        assert!(undo_result.is_ok());
        assert!(!app.flow_state.teams.values().any(|team| team.name == "Development"));

        let redo_result = app.redo(timestamp.date_naive());
        assert!(redo_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == "Development"));
    }

    #[test]
    fn test_create_rename_delete_team() {
        let mut app = Project::new("test_project.yaml");
        let timestamp = Utc::now();
        let team_name = "Development".to_string();
        let new_team_name = "Engineering".to_string();

        let create_result = app.invoke_command(Command { timestamp, details: CommandDetails::CreateTeam { name: team_name.clone() } }, timestamp.date_naive());
        assert!(create_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == team_name));

        let rename_result = app.invoke_command(Command { timestamp, details: CommandDetails::RenameTeam { old_name: team_name.clone(), new_name: new_team_name.clone() } }, timestamp.date_naive());
        assert!(rename_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == new_team_name));

        let delete_result = app.invoke_command(Command { timestamp, details: CommandDetails::DeleteTeam { name: new_team_name.clone() } }, timestamp.date_naive());
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

        let create_team_result = app.invoke_command(Command { timestamp, details: CommandDetails::CreateTeam { name: team_name.clone() } }, timestamp.date_naive());
        assert!(create_team_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == team_name));

        let create_resource_result = app.invoke_command(Command { timestamp, details: CommandDetails::CreateResource { name: resource_name.clone(), team_name: team_name.clone() } }, timestamp.date_naive());
        assert!(create_resource_result.is_ok());
        assert!(app.flow_state.resources.values().any(|res| res.name == resource_name));

        let rename_team_result = app.invoke_command(Command { timestamp, details: CommandDetails::RenameTeam { old_name: team_name.clone(), new_name: new_team_name.clone() } }, timestamp.date_naive());
        assert!(rename_team_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == new_team_name));

        let switch_team_result = app.invoke_command(Command { timestamp, details: CommandDetails::SwitchTeam { resource_name: resource_name.clone(), new_team_name: new_team_name.clone() } }, timestamp.date_naive());
        assert!(switch_team_result.is_ok());
        
        if let Some(resource) = app.flow_state.resources.get(&1) {
            assert_eq!(resource.team_id, 1); // Assuming the new team's ID is 1
        }

        let delete_resource_result = app.invoke_command(Command { timestamp, details: CommandDetails::DeleteResource { name: resource_name.clone() } }, timestamp.date_naive());
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
            Command { timestamp, details: CommandDetails::CreateTask {
                id: task_id,
                ticket,
                title: title.clone(),
                duration,
            }}, timestamp.date_naive());
        assert!(create_task_result.is_ok());
        assert!(app.flow_state.tasks.values().any(|task| task.title == title));

        let undo_result = app.undo(timestamp.date_naive());
        assert!(undo_result.is_ok());
        assert!(!app.flow_state.tasks.values().any(|task| task.title == title));

        let redo_result = app.redo(timestamp.date_naive());
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
        let create_team_result = app.invoke_command(Command { timestamp, details: CommandDetails::CreateTeam { name: team_name.clone() } }, timestamp.date_naive());
        assert!(create_team_result.is_ok());
        assert!(app.flow_state.teams.values().any(|team| team.name == team_name));

        // Create a resource in the team
        let create_resource_result = app.invoke_command(Command { timestamp, details: CommandDetails::CreateResource { name: resource_name.clone(), team_name: team_name.clone() } }, timestamp.date_naive());
        assert!(create_resource_result.is_ok());
        assert!(app.flow_state.resources.values().any(|res| res.name == resource_name));

        // Save to YAML
        app.save_to_yaml().unwrap();

        // Load from YAML
        if let Ok(loaded_app) = Project::load_from_yaml("database.yaml", NaiveDate::from_ymd_opt(2025, 8, 22).unwrap()) {
            // Verify loaded state
            assert!(loaded_app.flow_state.teams.values().any(|team| team.name == team_name));
            assert!(loaded_app.flow_state.resources.values().any(|res| res.name == resource_name));
        }

    }

    #[test]
    fn test_absence_intersections() {
        let a1 = Absence {
            create_timestamp: Utc::now(),
            start_date: NaiveDate::from_ymd_opt(2025, 8, 22).unwrap(),
            duration: TaskDuration { days: 1, fraction: 50 },
        };
        let a2 = Absence {
            create_timestamp: Utc::now(),
            start_date: NaiveDate::from_ymd_opt(2025, 8, 25).unwrap(),
            duration: TaskDuration { days: 0, fraction: 0 },
        };
        assert_eq!(a1.intersects(&a2), true);
    }

    #[test]
    fn test_alloc_cursor_add_assign_task_duration() {
        let mut cursor = AllocCursor::new(NaiveDate::from_ymd_opt(2025, 8, 22).unwrap());
        cursor += TaskDuration { days: 0, fraction: 50 };
        assert_eq!(cursor.alloced_amount, TaskDuration { days: 0, fraction: 50 });
    }
}
