use super::{FlowState, Command};

impl FlowState {
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
