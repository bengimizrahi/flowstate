use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    CreateTeam(TeamName),
    RenameTeam(TeamName, TeamName),
    DeleteTeam(TeamName),

    CreateResource(Resource),
    RenameResource(ResourceName, ResourceName),
    DeleteResource(ResourceName),

    CreateTask(Task),
    UpdateTask(Task),
    DeleteTask(TaskId),
    
    CompoundCommand(Vec<Command>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRecord {
    pub undo_command: Command,
    pub redo_command: Command,
}
