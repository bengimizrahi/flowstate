use crate::gui::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct GuiConfig {
    pub config_filename: String,
    pub ticket_prefix: String,
    pub hide_worklogs: bool,
    pub hide_weekends_in_inspection: bool,
    pub hide_non_deviations_in_inspection: bool,
    pub debug_mode: bool,
    pub recent_project_files: Vec<String>,
}

impl GuiConfig {
    pub fn load_from_yaml(path: &str) -> Self {
        if let Ok(contents) = std::fs::read_to_string(path) {
            if let Ok(config) = serde_yaml::from_str::<GuiConfig>(&contents) {
                return config;
            }
        }
        GuiConfig {
            config_filename: path.to_string(),
            ticket_prefix: "PROJ-".to_string(),
            hide_worklogs: false,
            hide_weekends_in_inspection: false,
            hide_non_deviations_in_inspection: false,
            debug_mode: false,
            recent_project_files: Vec::new(),
        }
    }

    pub fn save_to_file(&self) {
        if let Ok(contents) = serde_yaml::to_string(self) {
            let _ = std::fs::write(&self.config_filename, contents);
        }
    }
}