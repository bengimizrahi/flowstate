use crate::gui::*;
use crate::gui_log;

impl Gui {
    pub(super) fn draw_menu_bar(&mut self, ui: &Ui) {
        if ui.is_key_pressed(Key::Z) && ui.io().key_ctrl {
            /* let date = today as NaiveDate */
            self.project.undo(self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                gui_log!(self, "Failed to undo: {e}");
            });
        }
        if ui.is_key_pressed(Key::Y) && ui.io().key_ctrl {
            self.project.redo(self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                gui_log!(self, "Failed to redo: {e}");
            });
        }
        if let Some(_menu_bar) = ui.begin_menu_bar() {
            if let Some(_file_menu) = ui.begin_menu("File") {
                if let Some(_new_project_menu) = ui.begin_menu("New Project") {
                    if let Some(_child_window) = ui.child_window("##new_project_menu")
                            .size(NEW_PROJECT_CHILD_WINDOW_SIZE)
                            .begin() {
                        ui.input_text("##new_project_name", &mut self.new_project_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter project name")
                            .build();
                        ui.same_line();
                        if ui.button("Ok") {
                            ui.close_current_popup();
                            self.project = Project::new(&self.new_project_input_text_buffer);
                            gui_log!(self, "Created new project");
                            if !self.gui_config.recent_project_files.contains(&self.new_project_input_text_buffer) {
                                self.gui_config.recent_project_files.push(self.new_project_input_text_buffer.clone());
                                self.gui_config.save_to_file();
                            }
                            self.new_project_input_text_buffer.clear();
                        }
                    }
                }
                if ui.menu_item("Open Project...") {
                    if let Some(file_path) = rfd::FileDialog::new()
                        .add_filter("YAML files", &["yaml", "yml"])
                        .set_directory(".")
                        .pick_file() 
                    {
                        let file_path_str = file_path.to_string_lossy().to_string();
                        match Project::load_from_yaml(&file_path_str, self.get_timestamp().date_naive()) {
                            Ok(project) => {
                                if !self.gui_config.recent_project_files.contains(&file_path_str) {
                                    self.gui_config.recent_project_files.push(file_path_str.clone());
                                    self.gui_config.save_to_file();
                                }
                                self.project = project;
                                gui_log!(self, "Opened project from {file_path_str}");
                            },
                            Err(e) => {
                                gui_log!(self, "Failed to open project from {file_path_str}: {e}");
                            }
                        }
                    }
                }
                if let Some(_open_recent_menu) = ui.begin_menu("Open Recent") {
                    let recent_files = self.gui_config.recent_project_files.clone();
                    for recent_file in &recent_files {
                        if ui.menu_item(recent_file) {
                            match Project::load_from_yaml(recent_file, self.get_timestamp().date_naive()) {
                                Ok(project) => {
                                    self.project = project;
                                    gui_log!(self, "Opened project from {recent_file}");
                                    if let Some(pos) = self.gui_config.recent_project_files.iter().position(|f| f == recent_file) {
                                        self.gui_config.recent_project_files.remove(pos);
                                        self.gui_config.recent_project_files.insert(0, recent_file.clone());
                                        self.gui_config.save_to_file();
                                    }
                                },
                                Err(e) => {
                                    gui_log!(self, "Failed to open project from {recent_file}: {e}");
                                }
                            }
                        }
                    }
                }
                ui.separator();
                if ui.menu_item("Exit") {
                    std::process::exit(0);
                }
            };
            if let Some(_edit_menu) = ui.begin_menu("Edit") {
                if ui.menu_item_config("Undo").shortcut("Ctrl+Z").build() {
                    self.project.undo(self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                        gui_log!(self, "Failed to undo: {e}");
                    });
                }
                if ui.menu_item_config("Redo").shortcut("Ctrl+Y").build() {
                    self.project.redo(self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                        gui_log!(self, "Failed to redo: {e}");
                    });
                }
            };
            if let Some(_action_menu) = ui.begin_menu("Command") {
                if let Some(_team_menu) = ui.begin_menu("Team") {
                    if let Some(_child_window) = ui.child_window("##team_menu")
                            .size(CREATE_TEAM_CHILD_WINDOW_SIZE)
                            .begin() {
                        let mut can_create_team = false;
                        if ui.input_text("##team_name", &mut self.team_input_text_buffer)
                                .enter_returns_true(true)
                                .hint("Enter team name")
                                .build() {
                            can_create_team = !self.team_input_text_buffer.is_empty();
                        }
                        ui.same_line();
                        if ui.button("Ok") {
                            can_create_team = !self.team_input_text_buffer.is_empty();
                        }
                        if can_create_team {
                            ui.close_current_popup();
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::CreateTeam {
                                name: self.team_input_text_buffer.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap();
                            self.team_input_text_buffer.clear();
                        }
                    }
                }
                if let Some(_milestones_menu) = ui.begin_menu("Milestones") {
                    if let Some(_add_milestone_menu) = ui.begin_menu("Add Milestone") {
                        if let Some(_child_window) = ui.child_window("##milestone_menu")
                                .size(CREATE_MILESTONE_CHILD_WINDOW_SIZE)
                                .begin() {
                            let mut can_create_milestone = false;
                            if ui.input_text("##milestone_title", &mut self.milestone_input_text_buffer)
                                    .enter_returns_true(true)
                                    .hint("Milestone Title")
                                    .build() {
                                can_create_milestone = {
                                    !self.milestone_input_text_buffer.is_empty() &&
                                    !self.milestone_date_input_text_buffer.is_empty()
                                };
                            }
                            if ui.input_text("##milestone_date", &mut self.milestone_date_input_text_buffer)
                                    .enter_returns_true(true)
                                    .hint("Milestone Date (YYYY-MM-DD)")
                                    .build() {
                                can_create_milestone = {
                                    !self.milestone_input_text_buffer.is_empty() &&
                                    !self.milestone_date_input_text_buffer.is_empty()
                                };
                            }
                            ui.same_line();
                            if ui.button("Ok") {
                                can_create_milestone = {
                                    !self.milestone_input_text_buffer.is_empty() &&
                                    !self.milestone_date_input_text_buffer.is_empty()
                                };
                            }
                            if can_create_milestone {
                                ui.close_current_popup();
                                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddMilestone {
                                    title: self.milestone_input_text_buffer.clone(),
                                    date: NaiveDate::parse_from_str(&self.milestone_date_input_text_buffer, "%Y-%m-%d").unwrap(),
                                }}, self.get_timestamp().date_naive()).unwrap();
                                self.milestone_input_text_buffer.clear();
                            }
                        }
                    }
                    if let Some(_remove_milestone_menu) = ui.begin_menu("Remove Milestone") {
                        let milestones: Vec<_> = self.project.flow_state().milestones.iter().cloned().collect();
                        for milestone in milestones {
                            let milestone_label = format!("{} - {}", milestone.date.format("%Y-%m-%d"), milestone.title);
                            if ui.menu_item(&milestone_label) {
                                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveMilestone {
                                    title: milestone.title.clone(),
                                }}, self.get_timestamp().date_naive()).unwrap();
                            }
                        }
                    }
                }
            };
            if let Some(_label_menu) = ui.begin_menu("Label") {
                let labels: Vec<_> = self.project.flow_state().labels.iter().map(|(id, label)| (*id, label.clone())).collect();
                for (label_id, label) in labels {
                    let is_selected = self.filtered_labels.contains(&label_id);
                    if ui.menu_item_config(&label.name).selected(is_selected).build() {
                        if is_selected {
                            self.filtered_labels.retain(|&id| id != label_id);
                        } else {
                            self.filtered_labels.push(label_id);
                        }
                        self.selected_filter = None;
                    }
                }
                if ui.menu_item("Clear all") {
                    self.filtered_labels.clear();
                    self.selected_filter = None;
                }
            }
            if let Some(_filters_menu) = ui.begin_menu("Filter") {
                let filters: Vec<_> = self.project.flow_state().filters.iter().map(|(id, filter)| (*id, filter.clone())).collect();
                for (filter_id, filter) in &filters {
                    let is_selected = self.selected_filter == Some(*filter_id);
                    if ui.menu_item_config(&filter.name.clone()).selected(is_selected).build() {
                        if is_selected {
                            self.selected_filter = None;
                            self.filtered_labels.clear();
                        } else {
                            self.selected_filter = Some(*filter_id);
                            self.filtered_labels = filter.labels.iter().cloned().collect();
                        }
                    }
                }
                ui.separator();
                if let Some(_save_filter_menu) = ui.begin_menu("Save") {
                    for (filter_id, filter) in &filters {
                        if ui.menu_item(&filter.name) {
                            let label_names: Vec<String> = self.filtered_labels.iter()
                                .filter_map(|&label_id| self.project.flow_state().labels.get(&label_id))
                                .map(|label| label.name.clone())
                                .collect();
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::CreateModifyFilter {
                                name: filter.name.clone(),
                                labels: label_names,
                                is_favorite: filter.is_favorite,
                            }}, self.get_timestamp().date_naive()).unwrap();
                            self.selected_filter = Some(*filter_id);
                        }
                    }
                }
                if let Some(_save_as_filter_menu) = ui.begin_menu("Save as...") {
                    ui.input_text("##filter_name", &mut self.filter_input_text_buffer)
                        .enter_returns_true(true)
                        .hint("Enter filter name")
                        .build();
                    if ui.button("Ok") {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::CreateModifyFilter {
                            name: self.filter_input_text_buffer.clone(),
                            labels: self.filtered_labels.iter()
                                .filter_map(|&label_id| self.project.flow_state().labels.get(&label_id))
                                .map(|label| label.name.clone())
                                .collect(),
                            is_favorite: false,
                        }}, self.get_timestamp().date_naive()).unwrap();
                        let filter_id = self.project.flow_state().filters.iter()
                            .find(|(_, f)| f.name == self.filter_input_text_buffer)
                            .map(|(id, _)| *id);
                        self.selected_filter = filter_id;
                        self.filter_input_text_buffer.clear();
                    }
                }
                if let Some(_delete_filter_menu) = ui.begin_menu("Delete") {
                    for (filter_id, filter) in &filters {
                        let is_selected = self.selected_filter == Some(*filter_id);
                        if ui.menu_item(&filter.name) {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::DeleteFilter {
                                name: filter.name.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap();
                            if is_selected {
                                self.selected_filter = None;
                                self.filtered_labels.clear();
                            }
                        }
                    }
                }
                if let Some(_favorite_filter_menu) = ui.begin_menu("Favorites") {
                    for (_filter_id, filter) in &filters {
                        let is_favorite = filter.is_favorite;
                        if ui.menu_item_config(&filter.name).selected(is_favorite).build() {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::CreateModifyFilter {
                                name: filter.name.clone(),
                                labels: filter.labels.iter()
                                    .filter_map(|&label_id| self.project.flow_state().labels.get(&label_id))
                                    .map(|label| label.name.clone())
                                    .collect(),
                                is_favorite: !is_favorite,
                            }}, self.get_timestamp().date_naive()).unwrap();
                        }
                    }
                }
            }
            if let Some(_filters_menu) = ui.begin_menu("View") {
                if ui.menu_item_config("Hide Worklogs").selected(self.gui_config.hide_worklogs).build() {
                    self.gui_config.hide_worklogs = !self.gui_config.hide_worklogs;
                    self.gui_config.save_to_file();
                }
                if ui.menu_item_config("Hide Weekends in Inspection").selected(self.gui_config.hide_weekends_in_inspection).build() {
                    self.gui_config.hide_weekends_in_inspection = !self.gui_config.hide_weekends_in_inspection;
                    self.gui_config.save_to_file();
                }
                if ui.menu_item_config("Hide Non-Deviations in Inspection").selected(self.gui_config.hide_non_deviations_in_inspection).build() {
                    self.gui_config.hide_non_deviations_in_inspection = !self.gui_config.hide_non_deviations_in_inspection;
                    self.gui_config.save_to_file();
                }
            }
            if let Some(_help_menu) = ui.begin_menu("Help") {
                if ui.menu_item("About") {

                }
                ui.separator();
                if ui.menu_item_config("Debug Mode").selected(self.gui_config.debug_mode).build() {
                    self.gui_config.debug_mode = !self.gui_config.debug_mode;
                    self.gui_config.save_to_file();
                }
            }
        };
    }
}