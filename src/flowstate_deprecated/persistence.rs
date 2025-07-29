use super::{FlowState, Command};

impl FlowState {
    pub fn save_as_yaml(&self) -> Result<(), String> {
        let commands_to_serialize: Vec<Command> = self.command_history
            .iter()
            .map(|record| record.redo_command.clone())
            .collect();
        let data = (self.command_count, commands_to_serialize);
        let data_as_yaml = serde_yaml::to_string(&data)
            .map_err(|e| e.to_string())?;
        std::fs::write("database.yaml", data_as_yaml)
            .map_err(|e| e.to_string())
    }

    pub fn load_from_yaml(&mut self) -> Result<(), String> {
        let mut new_flow_state = FlowState::new();
        if let Ok(data_as_yaml) = std::fs::read_to_string("database.yaml") {
            if let Ok((command_count, commands)) = serde_yaml::from_str::<(usize, Vec<Command>)>(&data_as_yaml) {
                new_flow_state.command_count = command_count;
                commands.iter()
                    .try_for_each(|command| new_flow_state.invoke(command))?;
                
                let commands_to_undo = commands.len() - command_count;
                for _ in 0..commands_to_undo {
                    new_flow_state.undo()?;
                }
            }
        }
        *self = new_flow_state;
        dbg!(self);
        Ok(())
    }
}
