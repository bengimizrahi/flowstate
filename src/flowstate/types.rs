use serde::{Serialize, Deserialize};

pub type TeamName = String;
pub type ResourceName = String;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Resource {
    pub name: ResourceName,
    pub team_name: TeamName,
}
