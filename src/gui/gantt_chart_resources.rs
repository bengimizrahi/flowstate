use crate::gui::*;
use crate::gui_log;

/// Drag-and-drop payload name for moving tasks between resource rows in the resources Gantt.
/// ImGui limits this string to fewer than 32 bytes including the trailing NUL.
const GANTT_RESOURCES_TASK_DRAG: &str = "FS_GANTT_RES_TASK";

impl Gui {
    pub(super) fn draw_gantt_chart_resources(&mut self, ui: &Ui) {
        if self.draw_gantt_chart_table(ui, "##resources_gantt_chart") {
            self.draw_gantt_chart_calendar_row(ui);
            self.draw_gantt_chart_milestones_row(ui);
            self.draw_gantt_chart_resources_contents(ui);
            unsafe {imgui::sys::igEndTable();}
        }
    }

    fn draw_gantt_chart_resources_contents(&mut self, ui: &Ui) {
        let team_ids: Vec<TeamId> = self.project.flow_state().teams.iter()
            .map(|team| team.0.clone()).collect();
        for team_id in team_ids.iter(){
            self.draw_gantt_chart_resources_team(ui, team_id);
        }
        self.draw_gantt_chart_resources_team_unassigned(ui);
    }

    fn draw_gantt_chart_resources_team(&mut self, ui: &Ui, team_id: &TeamId) {
        ui.table_next_row();
        ui.table_next_column();

        let team = self.project.flow_state().teams.get(team_id).unwrap().clone();
        let team_name_cstr = std::ffi::CString::new(team.name.clone()).unwrap();
        let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_DefaultOpen;
        let expand_team = unsafe {
            let bold = self.bold_font.borrow().unwrap();
            let _h = ui.push_font(bold);
            let bg_color = ui.style_color(StyleColor::TableHeaderBg);
            ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
            imgui::sys::igTreeNodeEx_Str(team_name_cstr.as_ptr(), flags as i32)
        };
        self.draw_gantt_chart_resources_team_popup(ui, team_id, &team);

        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let bg_color = ui.style_color(StyleColor::TableHeaderBg);
                ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
                let day = self.project.flow_state().cache().day(i - 1);
                self.draw_milestone(ui, &day);
            }
        }

        if expand_team {
            let mut resources: Vec<ResourceId> = self.project.flow_state().teams[team_id]
                .resources.iter().map(|r| r.clone()).collect();
            resources.sort_by_key(|r| self.project.flow_state().resources[r].name.clone());
            for (i, resource_id) in resources.iter().enumerate() {
                self.drawing_aids.row_counter = i;
                self.draw_gantt_chart_resources_team_resource(ui, resource_id);
            }
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_gantt_chart_resources_team_resource(&mut self, ui: &Ui, resource_id: &ResourceId) {
        ui.table_next_row();
        ui.table_next_column();
        let resource = self.project.flow_state().resources.get(resource_id).unwrap().clone();
        let resource_name_cstr = std::ffi::CString::new(resource.name.clone()).unwrap();
        let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_DefaultOpen;
        let expand_resource = unsafe {
            let bold = self.bold_font.borrow().unwrap();
            let _h = ui.push_font(bold);
            let bg_color = if self.drawing_aids.row_counter % 2 == 0 {
                ui.style_color(StyleColor::TableRowBg)
            } else {
                ui.style_color(StyleColor::TableRowBgAlt)
            };
            ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
            imgui::sys::igTreeNodeEx_Str(resource_name_cstr.as_ptr(), flags as i32)
        };
        self.draw_gantt_chart_resources_team_resource_popup(ui, resource_id, &resource);

        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let _id = ui.push_id_usize(i);
                let day = self.project.flow_state().cache().day(i - 1);
                self.draw_cell_background(ui, &day);
                self.draw_absence(ui, &day, resource_id, &resource);
                if !self.gui_config.hide_worklogs {
                    self.draw_worklog_on_others_tasks(ui, &day, resource_id, &resource);
                }
                self.draw_milestone(ui, &day);
                ui.invisible_button("##invisible_button", [-1.0, unsafe { igGetTextLineHeight() }]);
                self.draw_gantt_chart_resources_team_resource_content_popup(ui, resource_id, &resource, &day);
            }
        }
        if expand_resource {
            for task_id in resource.assigned_tasks.iter() {
                let task = self.project.flow_state().tasks.get(task_id).unwrap().clone();
                let should_show = (
                    self.filtered_labels.is_empty()
                        || self.filtered_labels.iter().all(|label_id| task.label_ids.contains(label_id))
                ) && (
                    self.find_input_buffer.is_empty()
                        || task.title.contains(&self.find_input_buffer)
                        || task.ticket.contains(&self.find_input_buffer)
                );
                if should_show {
                    self.draw_gantt_chart_resources_team_resource_task(ui, resource_id, &resource, task_id);
                }
            }
            for task_id in resource.watched_tasks.iter() {
                let task = self.project.flow_state().tasks.get(task_id).unwrap().clone();
                let should_show = (
                    self.filtered_labels.is_empty()
                        || self.filtered_labels.iter().all(|label_id| task.label_ids.contains(label_id))
                ) && (
                    self.find_input_buffer.is_empty()
                        || task.title.contains(&self.find_input_buffer)
                        || task.ticket.contains(&self.find_input_buffer)
                );
                if should_show {
                    self.draw_gantt_chart_resources_team_resource_task_as_watcher(ui, resource_id, &resource, task_id);
                }
            }
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_gantt_chart_resources_team_resource_task(&mut self, ui: &Ui, resource_id: &ResourceId, resource: &Resource, task_id: &TaskId) {
        ui.table_next_row();
        ui.table_next_column();
        let _task_token_id = ui.push_id_int(*task_id as i32);
        let task = self.project.flow_state().tasks.get(task_id).unwrap().clone();
        let task_title_cstr = std::ffi::CString::new(format!("{} - {}", task.ticket, task.title)).unwrap();
        let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_Bullet;
        let expand_task = unsafe {
            let bg_color = if self.drawing_aids.row_counter % 2 == 0 {
                ui.style_color(StyleColor::TableRowBg)
            } else {
                ui.style_color(StyleColor::TableRowBgAlt)
            };
            ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
            imgui::sys::igTreeNodeEx_Str(task_title_cstr.as_ptr(), flags as i32)
        };

        if let Some(_tooltip) = ui.drag_drop_source_config(GANTT_RESOURCES_TASK_DRAG).begin_payload(*task_id) {}

        if let Some(target) = ui.drag_drop_target() {
            if let Some(Ok(payload)) =
                target.accept_payload::<TaskId, _>(GANTT_RESOURCES_TASK_DRAG, DragDropFlags::empty())
            {
                if payload.delivery {
                    let dragged_task_id = payload.data;
                    if dragged_task_id != *task_id {
                        self.project
                            .invoke_command(
                                Command {
                                    timestamp: self.get_timestamp(),
                                    details: CommandDetails::AssignTask {
                                        task_id: dragged_task_id,
                                        resource_name: resource.name.clone(),
                                    },
                                },
                                self.get_timestamp().date_naive(),
                            )
                            .unwrap_or_else(|e| {
                                gui_log!(self, "Failed to assign task via drag-drop: {e}");
                            });
                    }
                }
            }
            target.pop();
        }

        if ui.is_item_hovered() && ui.is_mouse_clicked(MouseButton::Middle) {
            self.open_task_in_jira(ui, &task);
        }
        self.draw_gantt_chart_resources_team_resource_task_popup(ui, task_id, &task);

        self.drawing_aids.previous_rect = None;
        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let _day_token_id = ui.push_id_usize(i);
                let day = self.project.flow_state().cache().day(i - 1);
                self.draw_cell_background(ui, &day);
                if !self.gui_config.hide_worklogs {
                    self.draw_worklog(ui, &day, resource_id, resource, task_id, &task);
                }
                
                let absence = self.project.flow_state().cache().resource_absence_rendering.get(resource_id)
                    .and_then(|r| r.get(&day)).copied();
                let worklog = self.project.flow_state().worklogs.get(task_id)
                    .and_then(|r| r.get(resource_id))
                    .and_then(|r| r.get(&day)).cloned();
                let alloc = 
                    self.project.flow_state().cache().task_alloc_rendering.get(task_id)
                        .and_then(|r| r.get(&day)).copied();
                self.draw_alloc(ui, worklog.clone(), alloc);
                self.draw_milestone(ui, &day);
                ui.invisible_button("##invisible_button", [-1.0, unsafe { igGetTextLineHeight() }]);
                self.draw_gantt_chart_resources_team_resource_task_content_popup(ui, resource_id, &resource, task_id, &task, &day);
                if ui.is_item_hovered() {
                    if absence.is_some() || worklog.is_some() || alloc.is_some() {
                        let _tooltip = ui.begin_tooltip();
                        if let Some(absence) = absence {
                            ui.bullet_text(format!("Absence: {}%", absence));
                        }
                        if let Some(worklog) = worklog {
                            ui.bullet_text(format!("Worklog: {}%", worklog.fraction));
                        }
                        if let Some(alloc) = alloc {
                            ui.bullet_text(format!("Alloc: {}%", alloc));
                        }
                    }
                }
            }
        }

        if expand_task {
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_gantt_chart_resources_team_resource_task_as_watcher(&mut self, ui: &Ui, resource_id: &ResourceId, resource: &Resource, task_id: &TaskId) {
        ui.table_next_row();
        ui.table_next_column();
        let _watcher_token_id = ui.push_id("##as_watcher");
        let _task_token_id = ui.push_id_int(*task_id as i32);
        let task = self.project.flow_state().tasks.get(task_id).unwrap().clone();
            let task_title_cstr = std::ffi::CString::new(format!("{} - {}", task.ticket, task.title)).unwrap();
        let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_Bullet;
        let expand_task = unsafe {
            let disabled_color = ui.style_color(StyleColor::TextDisabled);
            let _style = ui.push_style_color(imgui::StyleColor::Text, disabled_color);
            imgui::sys::igTreeNodeEx_Str(task_title_cstr.as_ptr(), flags as i32)
        };
        if ui.is_item_hovered() && ui.is_mouse_clicked(MouseButton::Middle) {
            self.open_task_in_jira(ui, &task);
        }
        self.draw_gantt_chart_resources_team_resource_task_as_watcher_popup(ui, task_id, &task, resource_id, resource);

        self.drawing_aids.previous_rect = None;
        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let _day_token_id = ui.push_id_usize(i);
                let day = self.project.flow_state().cache().day(i - 1);
                self.draw_cell_background(ui, &day);
                if !self.gui_config.hide_worklogs {
                    self.draw_worklog(ui, &day, resource_id, resource, task_id, &task);
                }
                self.draw_alloc_as_watcher(ui, &day, task.assignee.as_ref(), task_id, &task);
                self.draw_milestone(ui, &day);
                ui.invisible_button("##invisible_button", [-1.0, unsafe { igGetTextLineHeight() }]);
                self.draw_gantt_chart_resources_team_resource_task_content_popup(ui, resource_id, &resource, task_id, &task, &day);
            }
        }

        if expand_task {
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_gantt_chart_resources_team_unassigned(&mut self, ui: &Ui) {
        ui.table_next_row();
        ui.table_next_column();
        let name_cstr = std::ffi::CString::new("Unassigned").unwrap();
        let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_DefaultOpen;
        let expand_unassigned = unsafe {
            let bold = self.bold_font.borrow().unwrap();
            let _h = ui.push_font(bold);
            let bg_color = ui.style_color(StyleColor::TableHeaderBg);
            ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
            imgui::sys::igTreeNodeEx_Str(name_cstr.as_ptr(), flags as i32)
        };
        self.draw_gantt_chart_resources_team_unassigned_popup(ui);
        
        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let bg_color = ui.style_color(StyleColor::TableHeaderBg);
                ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
                let day = self.project.flow_state().cache().day(i - 1);
                self.draw_milestone(ui, &day);
            }
        }

        if expand_unassigned {
            for task_id in self.project.flow_state().cache().unassigned_tasks.clone().iter() {
                let task = self.project.flow_state().tasks.get(task_id).unwrap().clone();
                if self.filtered_labels.is_empty() || self.filtered_labels.iter().all(|label_id| task.label_ids.contains(label_id)) {
                    self.draw_gantt_chart_resources_team_unassigned_task(ui, task_id);
                }
            }

            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_gantt_chart_resources_team_unassigned_task(&mut self, ui: &Ui, task_id: &TaskId) {
        ui.table_next_row();
        ui.table_next_column();
        let _task_token_id = ui.push_id_int(*task_id as i32);
        let task = self.project.flow_state().tasks.get(task_id).unwrap().clone();
        let task_title_cstr = std::ffi::CString::new(format!("{} - {}", task.ticket, task.title)).unwrap();
        let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_Bullet;
        let expand_task = unsafe {
            imgui::sys::igTreeNodeEx_Str(task_title_cstr.as_ptr(), flags as i32)
        };
        if ui.is_item_hovered() && ui.is_mouse_clicked(MouseButton::Middle) {
            self.open_task_in_jira(ui, &task);
        }
        self.draw_gantt_chart_resources_team_unassigned_task_popup(ui, task_id, &task);

        self.drawing_aids.previous_rect = None;
        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let _day_token_id = ui.push_id_usize(i);
                let day = self.project.flow_state().cache().day(i - 1);
                self.draw_cell_background(ui, &day);
            let alloc = 
                self.project.flow_state().cache().task_alloc_rendering.get(task_id)
                    .and_then(|r| r.get(&day)).copied();

                self.draw_alloc(ui, None, alloc);
                self.draw_milestone(ui, &day);
                ui.invisible_button("##invisible_button", [-1.0, unsafe { igGetTextLineHeight() }]);
                self.draw_gantt_chart_resources_team_unassigned_task_content_popup(ui, task_id, &task, &day);
            }
        }
        if expand_task {
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_gantt_chart_resources_team_popup(&mut self, ui: &Ui, _team_id: &TeamId, team: &Team) {
        if let Some(_popup) = ui.begin_popup_context_item() {
            if let Some(_rename_team_menu) = ui.begin_menu("Rename Team") {
                if let Some(_child_window) = ui.child_window("##rename_team_menu")
                        .size(RENAME_TEAM_CHILD_WINDOW_SIZE)
                        .begin() {
                    let mut can_create_team = false;
                    if ui.input_text("##new_team_name", &mut self.team_input_text_buffer)
                            .enter_returns_true(true)
                            .hint(team.name.clone())
                            .build() {
                        can_create_team = !self.team_input_text_buffer.is_empty();
                    }
                    ui.same_line();
                    if ui.button("Ok") {
                        can_create_team = !self.team_input_text_buffer.is_empty();
                    }
                    if can_create_team {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RenameTeam {
                            old_name: team.name.clone(),
                            new_name: self.team_input_text_buffer.clone(),
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to rename team: {e}");
                        });
                        self.team_input_text_buffer.clear();
                    }
                }
            }
            if ui.menu_item("Delete Team") {
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::DeleteTeam {
                    name: team.name.clone(),
                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                    eprintln!("Failed to delete team: {e}");
                });
            }
            ui.separator();
            if let Some(_create_resource_menu) = ui.begin_menu("Create Resource") {
                if let Some(_child_window) = ui.child_window("##create_resource_menu")
                        .size(CREATE_RESOURCE_CHILD_WINDOW_SIZE)
                        .begin() {
                    let mut can_create_resource = false;
                    if ui.input_text("##resource_name", &mut self.resource_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter resource name")
                            .build() {
                        can_create_resource = !self.resource_input_text_buffer.is_empty();
                    }
                    ui.same_line();
                    if ui.button("Ok") {
                        can_create_resource = !self.resource_input_text_buffer.is_empty();
                    }
                    if can_create_resource {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::CreateResource {
                            name: self.resource_input_text_buffer.clone(),
                            team_name: team.name.clone(),
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to create resource: {e}");
                        });
                        self.resource_input_text_buffer.clear();
                    }
                }
            }
        }
    }

    pub(super) fn draw_gantt_chart_resources_team_resource_popup(&mut self, ui: &Ui, _resource_id: &ResourceId, resource: &Resource) {
        let is_info_filled_in =
                |task_title: &str, ticket: &str, duration: f32| {
            !task_title.is_empty() && !ticket.is_empty() && duration > 0.0
        };
        if let Some(_popup) = ui.begin_popup_context_item() {
            if let Some(_create_task_menu) = ui.begin_menu("Create Task") {
                if let Some(_child_window) = ui.child_window("##create_task_menu")
                        .size(CREATE_TASK_CHILD_WINDOW_SIZE)
                        .begin() {
                    if ui.input_text("##ticket", &mut self.ticket_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter ticket number")
                            .build() {
                    }
                    if ui.input_text("##task_title", &mut self.task_title_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter task title")
                            .build() {
                    }
                    ui.slider_config("##duration_slider", 0.0, 30.0)
                        .display_format("%.f days")
                        .build(&mut self.task_duration_days);
                    ui.input_float("##duration_input", &mut self.task_duration_days)
                        .display_format("%.2f days")
                        .step(1.0)
                        .build();
                    ui.disabled(!is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days), || {
                        if ui.button("Ok") {
                            ui.close_current_popup();
                            let task_id = self.project.flow_state_mut().next_task_id();
                            let timestamp = self.get_timestamp();
                            let mut commands = vec![
                                Command { timestamp, details: CommandDetails::CreateTask {
                                    id: task_id,
                                    ticket: self.ticket_input_text_buffer.clone(),
                                    title: self.task_title_input_text_buffer.clone(),
                                    duration: TaskDuration {
                                        days: self.task_duration_days as u64,
                                        fraction: (self.task_duration_days.fract() * 100.0) as u8,
                                    },
                                }},
                                Command { timestamp, details: CommandDetails::AssignTask {
                                    task_id: task_id,
                                    resource_name: resource.name.clone(),
                                }},
                            ];
                            for &label_id in &self.filtered_labels {
                                if let Some(label) = self.project.flow_state().labels.get(&label_id) {
                                    commands.push(Command { timestamp, details: CommandDetails::AddLabelToTask {
                                        task_id,
                                        label_name: label.name.clone(),
                                    }});
                                }
                            }
                            self.project.invoke_command(Command { timestamp, details: CommandDetails::CompoundCommand {
                                commands,
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                eprintln!("Failed to assign task: {e}");
                            });
                            self.task_title_input_text_buffer.clear();
                        }
                    });
                }
            }
            ui.separator();
            if let Some(_rename_resource_menu) = ui.begin_menu("Rename Resource") {
                if let Some(_child_window) = ui.child_window("##rename_resource_menu")
                        .size(RENAME_RESOURCE_CHILD_WINDOW_SIZE)
                        .begin() {
                    let mut can_create_resource = false;
                    if ui.input_text("##new_resource_name", &mut self.resource_input_text_buffer)
                            .enter_returns_true(true)
                            .hint(resource.name.clone())
                            .build() {
                        can_create_resource = !self.resource_input_text_buffer.is_empty();
                    }
                    ui.same_line();
                    if ui.button("Ok") {
                        can_create_resource = !self.resource_input_text_buffer.is_empty();
                    }
                    if can_create_resource {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RenameResource {
                            old_name: resource.name.clone(),
                            new_name: self.resource_input_text_buffer.clone(),
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to rename resource: {e}");
                        });
                        self.resource_input_text_buffer.clear();
                    }
                }
            }
            if ui.menu_item("Delete Resource") {
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::DeleteResource {
                    name: resource.name.clone(),
                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                    eprintln!("Failed to delete resource: {e}");
                });
            }
        }
    }

    pub(super) fn draw_gantt_chart_resources_team_resource_content_popup(&mut self, ui: &Ui, resource_id: &ResourceId, resource: &Resource, day: &NaiveDate) {
        let is_info_filled_in = |duration: f32| duration > 0.0;
        let add_or_update_absence_string;
        let mut show_remove_option = false;
        if let Some(_popup) = ui.begin_popup_context_item() {
            if self.project.flow_state().cache().resource_absence_rendering.get(resource_id).is_none() ||
                    self.project.flow_state().cache().resource_absence_rendering.get(resource_id).unwrap().get(day).is_none() ||
                    *self.project.flow_state().cache().resource_absence_rendering.get(resource_id).unwrap().get(day).unwrap() == 0 {
                add_or_update_absence_string = "Add Absence";
            } else {
                add_or_update_absence_string = "Update Absence";
                show_remove_option = true;
            }
            if let Some(_create_resource_menu) = ui.begin_menu(add_or_update_absence_string) {
                if let Some(_child_window) = ui.child_window("##add_or_update_absence_menu")
                        .size([270.0, 80.0])
                        .begin() {
                    let mut can_add_or_update_absence = false;
                    ui.slider_config("##duration", 0.1, 30.0)
                        .display_format("%.0f days")
                        .build(&mut self.absence_duration_days);
                    ui.input_float("##duration_input", &mut self.absence_duration_days)
                        .display_format("%.2f days")
                        .step(1.0)
                        .build();
                    ui.same_line();
                    if ui.button("Ok") {
                        can_add_or_update_absence = is_info_filled_in(self.absence_duration_days);
                    }
                    if ui.button("Half day") {
                        self.absence_duration_days = 0.5;
                        can_add_or_update_absence = is_info_filled_in(self.absence_duration_days);
                    }
                    ui.same_line();
                    if ui.button("1 day") {
                        self.absence_duration_days = 1.0;
                        can_add_or_update_absence = is_info_filled_in(self.absence_duration_days);
                    }
                    ui.same_line();
                    if ui.button("2 days") {
                        self.absence_duration_days = 2.0;
                        can_add_or_update_absence = is_info_filled_in(self.absence_duration_days);
                    }
                    ui.same_line();
                    if ui.button("1 week") {
                        self.absence_duration_days = 5.0;
                        can_add_or_update_absence = is_info_filled_in(self.absence_duration_days);
                    }
                    if can_add_or_update_absence {
                        ui.close_current_popup();
                        let absence_duration = TaskDuration {
                            days: self.absence_duration_days as u64,
                            fraction: (self.absence_duration_days.fract() * 100.0) as u8,
                        };
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::SetAbsence {
                            resource_name: resource.name.clone(),
                            start_date: *day,
                            days: absence_duration,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            gui_log!(self, "Failed to add Absence: {e}");
                        });
                        self.absence_duration_days = 0.0;
                    }
                }
            }
            if show_remove_option {
                if ui.menu_item("Remove Absence") {
                    self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::SetAbsence {
                        resource_name: resource.name.clone(),
                        start_date: *day,
                        days: TaskDuration {
                            days: 0,
                            fraction: 0,
                        },
                    }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                        gui_log!(self, "Failed to remove Absence: {e}");
                    });
                }
            }
        }
    }

    fn draw_gantt_chart_resources_team_resource_task_popup(&mut self, ui: &Ui, task_id: &TaskId, task: &Task) {
        if let Some(_popup) = ui.begin_popup_context_item() {
            if ui.menu_item("Move to top") {
                ui.close_current_popup();
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::PrioritizeTask {
                    task_id: *task_id,
                    to_top: true
                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to move task to top: {e}");
                });
            }
            if ui.menu_item("Move up") {
                ui.close_current_popup();
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::PrioritizeTask {
                    task_id: *task_id,
                    to_top: false
                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to move task up: {e}");
                });
            }
            if ui.menu_item("Move down") {
                ui.close_current_popup();
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::DeprioritizeTask {
                    task_id: *task_id,
                    to_bottom: false
                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to move task down: {e}");
                });
            }
            if ui.menu_item("Move to bottom") {
                ui.close_current_popup();
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::DeprioritizeTask {
                    task_id: *task_id,
                    to_bottom: true
                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to move task to bottom: {e}");
                });
            }
            ui.separator();
            if let Some(_assign_to_menu) = ui.begin_menu("Assign to") {
                let mut resources: Vec<_> = self.project.flow_state().resources.values().cloned().collect();
                resources.sort_by(|alloc, b| alloc.name.cmp(&b.name));
                for resource in resources {
                    if ui.menu_item(resource.name.clone()) {
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AssignTask {
                            task_id: *task_id,
                            resource_name: resource.name,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            gui_log!(self, "Failed to assign task to resource: {e}");
                        });
                    }
                }
            }
            if ui.menu_item("Unassign") {
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UnassignTask {
                    task_id: *task_id,
                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to unassign task: {e}");
                });
                ui.close_current_popup();
            }
            if let Some(_watchers_menu) = ui.begin_menu("Watchers") {
                /* list all the resources as alloc menu item. If the resource is already alloc watcher, it should be checked */
                let mut resources: Vec<_> = self.project.flow_state().resources.values().cloned().collect();
                resources.sort_by(|alloc, b| alloc.name.cmp(&b.name));
                for resource in resources {
                    let is_watching = resource.watched_tasks.contains(task_id);
                    if ui.menu_item_config(resource.name.clone()).selected(is_watching).build() {
                        if is_watching {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to unwatch task: {e}");
                            });
                        } else {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to watch task: {e}");
                            });
                        }
                    }
                }
            }
            ui.separator();
            if let Some(_update_task_menu) = ui.begin_menu("Update Task") {
                let is_info_filled_in =
                        |task_title: &str, ticket: &str, duration: f32| {
                    !task_title.is_empty() && !ticket.is_empty() && duration > 0.0
                };
                if let Some(_child_window) = ui.child_window("##update_task_menu")
                        .size(UPDATE_TASK_CHILD_WINDOW_SIZE)
                        .begin() {
                    let mut can_update_task = false;
                    if ui.input_text("##ticket", &mut self.ticket_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter ticket number")
                            .build() {
                        can_update_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    if ui.input_text("##task_title", &mut self.task_title_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter task title")
                            .build() {
                        can_update_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    ui.slider_config("##duration_slider", 0.1, 30.0)
                        .display_format("%.0f days")
                        .build(&mut self.task_duration_days);
                    ui.input_float("##duration_input", &mut self.task_duration_days)
                        .display_format("%.2f days")
                        .step(1.0)
                        .build();
                    if ui.button("Ok") {
                        can_update_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    if can_update_task {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: self.ticket_input_text_buffer.clone(),
                            title: self.task_title_input_text_buffer.clone(),
                            duration: TaskDuration {
                                days: self.task_duration_days as u64,
                                fraction: (self.task_duration_days.fract() * 100.0) as u8,
                            },
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                        self.task_title_input_text_buffer.clear();
                    }
                }
            }
            if ui.menu_item("Inspect") {
                self.inspections.push(TaskInspection::from(
                        *task_id,
                        self.project.command_stack.iter()
                            .map(|cr| cr.redo_command.clone())
                            .collect::<Vec<_>>(),
                        self.get_timestamp().date_naive())
                );
                ui.close_current_popup();
            }
            if ui.menu_item("Open in JIRA") {
                self.open_task_in_jira(ui, &task);
                ui.close_current_popup();
            }
            ui.separator();
            if let Some(_labels_menu) = ui.begin_menu("Labels") {
                let labels: Vec<_> = self.project.flow_state().labels.iter().map(|(id, label)| (*id, label.clone())).collect();
                for (label_id, label) in labels {
                    let is_selected = task.label_ids.contains(&label_id);
                    if ui.menu_item_config(&label.name).selected(is_selected).build() {
                        if is_selected {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveLabelFromTask {
                                task_id: *task_id,
                                label_name: label.name.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to remove label from task: {e}");
                            });
                        } else {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddLabelToTask {
                                task_id: *task_id,
                                label_name: label.name.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to add label to task: {e}");
                            });
                        }
                    }
                }
                ui.separator();
                if let Some(_new_label_menu) = ui.begin_menu("New Label") {
                    if let Some(_child_window) = ui.child_window("##new_label")
                            .size(CREATE_LABEL_CHILD_WINDOW_SIZE)
                            .begin() {
                        ui.input_text("##label_name", &mut self.label_input_text_buffer)
                            .hint("Enter label name")
                            .build();
                        if ui.button("Ok") && !self.label_input_text_buffer.is_empty() {
                            let timestamp = self.get_timestamp();
                            self.project.invoke_command(Command { timestamp, details: CommandDetails::CompoundCommand {
                                commands: vec![
                                    Command { timestamp, details: CommandDetails::CreateLabel {
                                        name: self.label_input_text_buffer.clone(),
                                    }},
                                    Command { timestamp, details: CommandDetails::AddLabelToTask {
                                        task_id: *task_id,
                                        label_name: self.label_input_text_buffer.clone(),
                                    }}
                                ]
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to create label and add to task: {e}");
                            });
                            self.label_input_text_buffer.clear();
                        }
                    }
                }
            }
            ui.separator();
            if let Some(_update_task_menu) = ui.begin_menu("Update Duration") {
                if let Some(_child_window) = ui.child_window("##update_duration_menu")
                        .size(UPDATE_TASK_CHILD_WINDOW_SIZE)
                        .begin() {
                    ui.slider_config("##duration", 0.0, 30.0)
                        .display_format("%.0f days")
                        .build(&mut self.task_duration_days);
                    if ui.button("Ok") {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: task.ticket.clone(),
                            title: task.title.clone(),
                            duration: TaskDuration {
                                days: self.task_duration_days as u64,
                                fraction: (self.task_duration_days.fract() * 100.0) as u8,
                            },
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    let mut new_duration_days = None;
                    if ui.button("<<") {
                        new_duration_days = Some(TaskDuration::zero()
                            .max(task.duration - TaskDuration { days: 5, fraction: 0 }));
                    }
                    ui.same_line();
                    if ui.button("<") {
                        new_duration_days = Some(TaskDuration::zero()
                            .max(task.duration - TaskDuration { days: 1, fraction: 0 }));
                    }
                    ui.same_line();
                    if ui.button(">") {
                        new_duration_days = Some(task.duration + TaskDuration { days: 1, fraction: 0 });
                    }
                    ui.same_line();
                    if ui.button(">>") {
                        new_duration_days = Some(task.duration + TaskDuration { days: 5, fraction: 0 });
                    }
                    if let Some(new_duration_days) = new_duration_days {
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: task.ticket.clone(),
                            title: task.title.clone(),
                            duration: new_duration_days,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                }
            }
        }
    }

    fn draw_gantt_chart_resources_team_resource_task_as_watcher_popup(&mut self, ui: &Ui, task_id: &TaskId, task: &Task, _resource_id: &ResourceId, resource: &Resource) {
        if let Some(_popup) = ui.begin_popup_context_item() {
            if let Some(_assign_to_menu) = ui.begin_menu("Assign to") {
                let mut resources: Vec<_> = self.project.flow_state().resources.values().cloned().collect();
                resources.sort_by(|alloc, b| alloc.name.cmp(&b.name));
                for resource in resources {
                    if ui.menu_item(resource.name.clone()) {
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AssignTask {
                            task_id: *task_id,
                            resource_name: resource.name,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            gui_log!(self, "Failed to assign task to resource: {e}");
                        });
                    }
                }
            }
            if ui.menu_item("Unwatch") {
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveWatcher {
                    task_id: *task_id,
                    resource_name: resource.name.clone(),
                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to unwatch task: {e}");
                });
                ui.close_current_popup();
            }
            if let Some(_watchers_menu) = ui.begin_menu("Watchers") {
                /* list all the resources as alloc menu item. If the resource is already alloc watcher, it should be checked */
                let mut resources: Vec<_> = self.project.flow_state().resources.values().cloned().collect();
                resources.sort_by(|alloc, b| alloc.name.cmp(&b.name));
                for resource in resources {
                    let is_watching = resource.watched_tasks.contains(task_id);
                    if ui.menu_item_config(resource.name.clone()).selected(is_watching).build() {
                        if is_watching {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to unwatch task: {e}");
                            });
                        } else {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to watch task: {e}");
                            });
                        }
                    }
                }
            }
            ui.separator();
            if let Some(_update_task_menu) = ui.begin_menu("Update Task") {
                let is_info_filled_in =
                        |task_title: &str, ticket: &str, duration: f32| {
                    !task_title.is_empty() && !ticket.is_empty() && duration > 0.0
                };
                if let Some(_child_window) = ui.child_window("##update_task_menu")
                        .size(UPDATE_TASK_CHILD_WINDOW_SIZE)
                        .begin() {
                    let mut can_update_task = false;
                    if ui.input_text("##ticket", &mut self.ticket_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter ticket number")
                            .build() {
                        can_update_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    if ui.input_text("##task_title", &mut self.task_title_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter task title")
                            .build() {
                        can_update_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    ui.slider_config("##duration_slider", 0.1, 30.0)
                        .display_format("%.0f days")
                        .build(&mut self.task_duration_days);
                    ui.input_float("##duration_input", &mut self.task_duration_days)
                        .display_format("%.2f days")
                        .step(1.0)
                        .build();
                    if ui.button("Ok") {
                        can_update_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    if can_update_task {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: self.ticket_input_text_buffer.clone(),
                            title: self.task_title_input_text_buffer.clone(),
                            duration: TaskDuration {
                                days: self.task_duration_days as u64,
                                fraction: (self.task_duration_days.fract() * 100.0) as u8,
                            },
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                        self.task_title_input_text_buffer.clear();
                    }
                }
            }
            if ui.menu_item("Open in JIRA") {
                self.open_task_in_jira(ui, &task);
                ui.close_current_popup();
            }
            ui.separator();
            if let Some(_labels_menu) = ui.begin_menu("Labels") {
                let labels: Vec<_> = self.project.flow_state().labels.iter().map(|(id, label)| (*id, label.clone())).collect();
                for (label_id, label) in labels {
                    let is_selected = task.label_ids.contains(&label_id);
                    if ui.menu_item_config(&label.name).selected(is_selected).build() {
                        if is_selected {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveLabelFromTask {
                                task_id: *task_id,
                                label_name: label.name.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to remove label from task: {e}");
                            });
                        } else {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddLabelToTask {
                                task_id: *task_id,
                                label_name: label.name.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to add label to task: {e}");
                            });
                        }
                    }
                }
                ui.separator();
                if let Some(_new_label_menu) = ui.begin_menu("New Label") {
                    if let Some(_child_window) = ui.child_window("##new_label")
                            .size(CREATE_LABEL_CHILD_WINDOW_SIZE)
                            .begin() {
                        ui.input_text("##label_name", &mut self.label_input_text_buffer)
                            .hint("Enter label name")
                            .build();
                        if ui.button("Ok") && !self.label_input_text_buffer.is_empty() {
                            let timestamp = self.get_timestamp();
                            self.project.invoke_command(Command { timestamp, details: CommandDetails::CompoundCommand {
                                commands: vec![
                                    Command { timestamp, details: CommandDetails::CreateLabel {
                                        name: self.label_input_text_buffer.clone(),
                                    }},
                                    Command { timestamp, details: CommandDetails::AddLabelToTask {
                                        task_id: *task_id,
                                        label_name: self.label_input_text_buffer.clone(),
                                    }}
                                ]
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to create label and add to task: {e}");
                            });
                            self.label_input_text_buffer.clear();
                        }
                    }
                }
            }
            ui.separator();
            if let Some(_update_task_menu) = ui.begin_menu("Update Duration") {
                if let Some(_child_window) = ui.child_window("##update_duration_menu")
                        .size(UPDATE_TASK_CHILD_WINDOW_SIZE)
                        .begin() {
                    ui.slider_config("##duration", 0.0, 30.0)
                        .display_format("%.0f days")
                        .build(&mut self.task_duration_days);
                    if ui.button("Ok") {
                        ui.close_current_popup();
                        let timestamp = self.get_timestamp();
                        self.project.invoke_command(Command { timestamp, details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: task.ticket.clone(),
                            title: task.title.clone(),
                            duration: TaskDuration {
                                days: self.task_duration_days as u64,
                                fraction: (self.task_duration_days.fract() * 100.0) as u8,
                            },
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    let mut new_duration_days = None;
                    if ui.button("<<") {
                        new_duration_days = Some(TaskDuration::zero()
                            .max(task.duration - TaskDuration { days: 5, fraction: 0 }));
                    }
                    ui.same_line();
                    if ui.button("<") {
                        new_duration_days = Some(TaskDuration::zero()
                            .max(task.duration - TaskDuration { days: 1, fraction: 0 }));
                    }
                    ui.same_line();
                    if ui.button(">") {
                        new_duration_days = Some(task.duration + TaskDuration { days: 1, fraction: 0 });
                    }
                    ui.same_line();
                    if ui.button(">>") {
                        new_duration_days = Some(task.duration + TaskDuration { days: 5, fraction: 0 });
                    }
                    if let Some(new_duration_days) = new_duration_days {
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: task.ticket.clone(),
                            title: task.title.clone(),
                            duration: new_duration_days,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                }
            }
        }
    }

    fn draw_gantt_chart_resources_team_resource_task_content_popup(&mut self, ui: &Ui, resource_id: &ResourceId, resource: &Resource, task_id: &TaskId, task: &Task, day: &NaiveDate) {
        if let Some(_popup) = ui.begin_popup_context_item() {
            if let Some(_worklog_menu) = ui.begin_menu("Set Worklog") {
                if let Some(_child_window) = ui.child_window("##set_worklog")
                        .size(SET_WORKLOG_CHILD_WINDOW_SIZE)
                        .begin() {
                    ui.slider_config("##fraction", 0, 100)
                        .build(&mut self.worklog_fraction);
                    ui.same_line();
                    if ui.button("Ok") {
                        ui.close_current_popup();
                        // Convert NaiveDate to DateTime<Utc> at midnight
                        let timestamp = DateTime::from_naive_utc_and_offset((*day).and_hms_opt(0, 0, 0).unwrap(), Utc);
                        self.project.invoke_command(Command { timestamp, details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: self.worklog_fraction,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    if ui.button("0%") {
                        ui.close_current_popup();
                        let timestamp = DateTime::from_naive_utc_and_offset((*day).and_hms_opt(0, 0, 0).unwrap(), Utc);
                        self.project.invoke_command(Command { timestamp, details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 0,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    ui.same_line();
                    if ui.button("10%") {
                        ui.close_current_popup();
                        let timestamp = DateTime::from_naive_utc_and_offset((*day).and_hms_opt(0, 0, 0).unwrap(), Utc);
                        self.project.invoke_command(Command { timestamp, details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 10,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    ui.same_line();
                    if ui.button("25%") {
                        ui.close_current_popup();
                        let timestamp = DateTime::from_naive_utc_and_offset((*day).and_hms_opt(0, 0, 0).unwrap(), Utc);
                        self.project.invoke_command(Command { timestamp, details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 25,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    ui.same_line();
                    if ui.button("50%") {
                        ui.close_current_popup();
                        let timestamp = DateTime::from_naive_utc_and_offset((*day).and_hms_opt(0, 0, 0).unwrap(), Utc);
                        self.project.invoke_command(Command { timestamp, details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 50,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    ui.same_line();
                    if ui.button("75%") {
                        ui.close_current_popup();
                        let timestamp = DateTime::from_naive_utc_and_offset((*day).and_hms_opt(0, 0, 0).unwrap(), Utc);
                        self.project.invoke_command(Command { timestamp, details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 75,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }                    
                    ui.same_line();
                    if ui.button("100%") {
                        ui.close_current_popup();
                        let timestamp = DateTime::from_naive_utc_and_offset((*day).and_hms_opt(0, 0, 0).unwrap(), Utc);
                        self.project.invoke_command(Command { timestamp, details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 100,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    if ui.button("Use all available") {
                        ui.close_current_popup();
                        let absence_fraction = self.project.flow_state().cache().resource_absence_rendering.get(resource_id)
                            .and_then(|r| r.get(day))
                            .cloned().unwrap_or(0);
                        let total_worklogs_for_resource_for_day = {
                            let mut total = 0;
                            for task_allocs in self.project.flow_state().worklogs.values() {
                                if let Some(resource_worklogs) = task_allocs.get(resource_id) {
                                    if let Some(worklog) = resource_worklogs.get(day) {
                                        total += worklog.fraction as u32;
                                    }
                                }
                            }
                            total
                        };
                        if let Some(remaining_fraction) = 100u32.checked_sub(absence_fraction as u32 + total_worklogs_for_resource_for_day) {
                            let current_worklog_fraction = self.project.flow_state().worklogs.get(task_id)
                                .and_then(|task_allocs| task_allocs.get(resource_id))
                                .and_then(|resource_worklogs| resource_worklogs.get(day))
                                .map(|w| w.fraction)
                                .unwrap_or(0);
                            let timestamp = DateTime::from_naive_utc_and_offset((*day).and_hms_opt(0, 0, 0).unwrap(), Utc);
                            self.project.invoke_command(Command { timestamp, details: CommandDetails::SetWorklog {
                                task_id: *task_id,
                                date: *day,
                                resource_name: resource.name.clone(),
                                fraction: current_worklog_fraction + remaining_fraction as u8,
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                eprintln!("Failed to update task: {e}");
                            });
                        }
                    }
                }
            }
            if let Some(_update_duration_menu) = ui.begin_menu("Update Duration") {
                if let Some(_child_window) = ui.child_window("##update_duration_menu")
                        .size(UPDATE_TASK_CHILD_WINDOW_SIZE)
                        .begin() {
                    ui.slider_config("##duration", 0.0, 30.0)
                        .display_format("%.0f days")
                        .build(&mut self.task_duration_days);
                    if ui.button("Ok") {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: task.ticket.clone(),
                            title: task.title.clone(),
                            duration: TaskDuration {
                                days: self.task_duration_days as u64,
                                fraction: (self.task_duration_days.fract() * 100.0) as u8,
                            },
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    let mut new_duration_days = None;
                    if ui.button("<<") {
                        new_duration_days = Some(TaskDuration::zero()
                            .max(task.duration - TaskDuration { days: 5, fraction: 0 }));
                    }
                    ui.same_line();
                    if ui.button("<") {
                        new_duration_days = Some(TaskDuration::zero()
                            .max(task.duration - TaskDuration { days: 1, fraction: 0 }));
                    }
                    ui.same_line();
                    if ui.button(">") {
                        new_duration_days = Some(task.duration + TaskDuration { days: 1, fraction: 0 });
                    }
                    ui.same_line();
                    if ui.button(">>") {
                        new_duration_days = Some(task.duration + TaskDuration { days: 5, fraction: 0 });
                    }
                    if new_duration_days.is_some() {
                        println!("New duration days: {:?}", new_duration_days);
                    }
                    if let Some(new_duration_days) = new_duration_days {
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: task.ticket.clone(),
                            title: task.title.clone(),
                            duration: new_duration_days,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    if ui.button("Crop") {
                        if let Some(task_alloc_for_day) = self.project.flow_state().cache().task_alloc_rendering.get(task_id)
                                .and_then(|r| r.get(day)).cloned() {
                            let duration = TaskDuration {
                                days: task.duration.days,
                                fraction: task.duration.fraction,
                            } - (TaskDuration {
                                days: 0,
                                fraction: task_alloc_for_day,
                            });
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                                id: *task_id,
                                ticket: task.ticket.clone(),
                                title: task.title.clone(),
                                duration,
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                eprintln!("Failed to crop task: {e}");
                            });
                        }
                    }
                    if ui.button("Round up") {
                        if let Some(task_alloc_for_day) = self.project.flow_state().cache().task_alloc_rendering.get(task_id)
                                .and_then(|r| r.get(day)).cloned() {
                            let duration = task.duration + (TaskDuration {
                                days: 0,
                                fraction: 100 - task_alloc_for_day,
                            });
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                                id: *task_id,
                                ticket: task.ticket.clone(),
                                title: task.title.clone(),
                                duration,
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                eprintln!("Failed to crop task: {e}");
                            });
                        }
                    }
                }
            }
        }
    }

    fn draw_gantt_chart_resources_team_unassigned_popup(&mut self, ui: &Ui) {
        let is_info_filled_in =
                |task_title: &str, ticket: &str, duration: f32| {
            !task_title.is_empty() && !ticket.is_empty() && duration > 0.0
        };
        if let Some(_popup) = ui.begin_popup_context_item() {
            if let Some(_create_task_menu) = ui.begin_menu("Create Task") {
                if let Some(_child_window) = ui.child_window("##create_task_menu")
                        .size(CREATE_TASK_CHILD_WINDOW_SIZE)
                        .begin() {
                    let mut can_create_task = false;
                    if ui.input_text("##ticket", &mut self.ticket_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter ticket number")
                            .build() {
                        can_create_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    if ui.input_text("##task_title", &mut self.task_title_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter task title")
                            .build() {
                        can_create_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    ui.slider_config("##duration_slider", 0.1, 30.0)
                        .display_format("%.f days")
                        .build(&mut self.task_duration_days);
                    ui.input_float("##duration_input", &mut self.task_duration_days)
                        .display_format("%.2f days")
                        .step(1.0)
                        .build();
                    if ui.button("Ok") {
                        can_create_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    if can_create_task {
                        ui.close_current_popup();
                        let task_id = self.project.flow_state_mut().next_task_id();
                        let mut commands = vec![
                            Command { timestamp: self.get_timestamp(), details: CommandDetails::CreateTask {
                                id: task_id,
                                ticket: self.ticket_input_text_buffer.clone(),
                                title: self.task_title_input_text_buffer.clone(),
                                duration: TaskDuration {
                                    days: self.task_duration_days as u64,
                                    fraction: (self.task_duration_days.fract() * 100.0) as u8,
                                },
                            }}
                        ];
                        for &label_id in &self.filtered_labels {
                            if let Some(label) = self.project.flow_state().labels.get(&label_id) {
                                commands.push(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddLabelToTask {
                                    task_id,
                                    label_name: label.name.clone(),
                                }});
                            }
                        }
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::CompoundCommand {
                            commands,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to create task: {e}");
                        });
                        self.task_title_input_text_buffer.clear();
                    }
                }
            }
        }
    }

    fn draw_gantt_chart_resources_team_unassigned_task_popup(&mut self, ui: &Ui, task_id: &TaskId, task: &Task) {
        if let Some(_popup) = ui.begin_popup_context_item() {
            if let Some(_assign_to_menu) = ui.begin_menu("Assign to") {
                let mut resources: Vec<_> = self.project.flow_state().resources.values().cloned().collect();
                resources.sort_by(|alloc, b| alloc.name.cmp(&b.name));
                for resource in resources {
                    if ui.menu_item(resource.name.clone()) {
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AssignTask {
                            task_id: *task_id,
                            resource_name: resource.name,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            gui_log!(self, "Failed to assign task to resource: {e}");
                        });
                    }
                }
            }
            if let Some(_watchers_menu) = ui.begin_menu("Watchers") {
                /* list all the resources as alloc menu item. If the resource is already alloc watcher, it should be checked */
                let mut resources: Vec<_> = self.project.flow_state().resources.values().cloned().collect();
                resources.sort_by(|alloc, b| alloc.name.cmp(&b.name));
                for resource in resources {
                    let is_watching = resource.watched_tasks.contains(task_id);
                    if ui.menu_item_config(resource.name.clone()).selected(is_watching).build() {
                        if is_watching {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to unwatch task: {e}");
                            });
                        } else {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to watch task: {e}");
                            });
                        }
                    }
                }
            }
            ui.separator();
            if ui.menu_item("Delete") {
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::DeleteTask {
                    id: *task_id,
                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to delete task: {e}");
                });
            }
            if let Some(_update_task_menu) = ui.begin_menu("Update Task") {
                let is_info_filled_in =
                        |task_title: &str, ticket: &str, duration: f32| {
                    !task_title.is_empty() && !ticket.is_empty() && duration > 0.0
                };
                if let Some(_child_window) = ui.child_window("##update_task_menu")
                        .size(UPDATE_TASK_CHILD_WINDOW_SIZE)
                        .begin() {
                    let mut can_update_task = false;
                    if ui.input_text("##ticket", &mut self.ticket_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter ticket number")
                            .build() {
                        can_update_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    if ui.input_text("##task_title", &mut self.task_title_input_text_buffer)
                            .enter_returns_true(true)
                            .hint("Enter task title")
                            .build() {
                        can_update_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    ui.slider_config("##duration_slider", 0.1, 30.0)
                        .display_format("%.0f days")
                        .build(&mut self.task_duration_days);
                    ui.input_float("##duration_input", &mut self.task_duration_days)
                        .display_format("%.2f days")
                        .step(1.0)
                        .build();
                    if ui.button("Ok") {
                        can_update_task = is_info_filled_in(
                            &self.task_title_input_text_buffer,
                            &self.ticket_input_text_buffer,
                            self.task_duration_days);
                    }
                    if can_update_task {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: self.ticket_input_text_buffer.clone(),
                            title: self.task_title_input_text_buffer.clone(),
                            duration: TaskDuration {
                                days: self.task_duration_days as u64,
                                fraction: (self.task_duration_days.fract() * 100.0) as u8,
                            },
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                        self.task_title_input_text_buffer.clear();
                    }
                }
            }
            if ui.menu_item("Open in JIRA") {
                self.open_task_in_jira(ui, &task);
                ui.close_current_popup();
            }
            ui.separator();
            if let Some(_update_task_menu) = ui.begin_menu("Labels") {
                if let Some(_add_label_menu) = ui.begin_menu("Add Label") {
                    let labels: Vec<_> = self.project.flow_state().labels.iter().map(|(id, label)| (*id, label.clone())).collect();
                    for (label_id, label) in labels {
                        if !task.label_ids.contains(&label_id) {
                            if ui.menu_item(&label.name) {
                                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddLabelToTask {
                                    task_id: *task_id,
                                    label_name: label.name.clone(),
                                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                    gui_log!(self, "Failed to add label to task: {e}");
                                });
                            }
                        }
                    }
                    if let Some(_update_task_menu) = ui.begin_menu("New Label") {
                        if let Some(_child_window) = ui.child_window("##new_label")
                                .size(CREATE_LABEL_CHILD_WINDOW_SIZE)
                                .begin() {
                            ui.input_text("##label_name", &mut self.label_input_text_buffer)
                                .hint("Enter label name")
                                .build();
                            if ui.button("Ok") && !self.label_input_text_buffer.is_empty() {
                                let timestamp = self.get_timestamp();
                                self.project.invoke_command(Command { timestamp, details: CommandDetails::CompoundCommand {
                                    commands: vec![
                                        Command { timestamp, details: CommandDetails::CreateLabel {
                                            name: self.label_input_text_buffer.clone(),
                                        }},
                                        Command { timestamp, details: CommandDetails::AddLabelToTask {
                                            task_id: *task_id,
                                            label_name: self.label_input_text_buffer.clone(),
                                        }}
                                    ]
                                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                    gui_log!(self, "Failed to create label and add to task: {e}");
                                });
                                self.label_input_text_buffer.clear();
                            }
                        }
                    }
                }
                if let Some(_remove_label_menu) = ui.begin_menu("Remove Label") {
                    let labels: Vec<_> = self.project.flow_state().labels.iter().map(|(id, label)| (*id, label.clone())).collect();
                    for (label_id, label) in labels {
                        if task.label_ids.contains(&label_id) {
                            if ui.menu_item(&label.name) {
                                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveLabelFromTask {
                                    task_id: *task_id,
                                    label_name: label.name.clone(),
                                }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                                    gui_log!(self, "Failed to remove label from task: {e}");
                                });
                            }
                        }
                    }
                }
            }
            ui.separator();
            if let Some(_update_duration_menu) = ui.begin_menu("Update Duration") {
                if let Some(_child_window) = ui.child_window("##update_duration_menu")
                        .size(UPDATE_TASK_CHILD_WINDOW_SIZE)
                        .begin() {
                    ui.slider_config("##duration", 0.0, 30.0)
                        .display_format("%.0f days")
                        .build(&mut self.task_duration_days);
                    if ui.button("Ok") {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: task.ticket.clone(),
                            title: task.title.clone(),
                            duration: TaskDuration {
                                days: self.task_duration_days as u64,
                                fraction: (self.task_duration_days.fract() * 100.0) as u8,
                            },
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    let mut new_duration_days = None;
                    if ui.button("<<") {
                        new_duration_days = Some(TaskDuration::zero()
                            .max(task.duration - TaskDuration { days: 7, fraction: 0 }));
                    }
                    ui.same_line();
                    if ui.button("<") {
                        new_duration_days = Some(TaskDuration::zero()
                            .max(task.duration - TaskDuration { days: 1, fraction: 0 }));
                    }
                    ui.same_line();
                    if ui.button(">") {
                        new_duration_days = Some(task.duration + TaskDuration { days: 1, fraction: 0 });
                    }
                    ui.same_line();
                    if ui.button(">>") {
                        new_duration_days = Some(task.duration + TaskDuration { days: 7, fraction: 0 });
                    }
                    if let Some(new_duration_days) = new_duration_days {
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: task.ticket.clone(),
                            title: task.title.clone(),
                            duration: new_duration_days,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                }
            }
        }
    }

    fn draw_gantt_chart_resources_team_unassigned_task_content_popup(&mut self, ui: &Ui, task_id: &TaskId, task: &Task, _day: &NaiveDate) {
        if let Some(_popup) = ui.begin_popup_context_item() {
            if let Some(_update_duration_menu) = ui.begin_menu("Update Duration") {
                if let Some(_child_window) = ui.child_window("##update_duration_menu")
                        .size(UPDATE_TASK_CHILD_WINDOW_SIZE)
                        .begin() {
                    ui.slider_config("##duration", 0.0, 30.0)
                        .display_format("%.0f days")
                        .build(&mut self.task_duration_days);
                    if ui.button("Ok") {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: task.ticket.clone(),
                            title: task.title.clone(),
                            duration: TaskDuration {
                                days: self.task_duration_days as u64,
                                fraction: (self.task_duration_days.fract() * 100.0) as u8,
                            },
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    let mut new_duration_days = None;
                    if ui.button("<<") {
                        new_duration_days = Some(TaskDuration::zero()
                            .max(task.duration - TaskDuration { days: 7, fraction: 0 }));
                    }
                    ui.same_line();
                    if ui.button("<") {
                        new_duration_days = Some(TaskDuration::zero()
                            .max(task.duration - TaskDuration { days: 1, fraction: 0 }));
                    }
                    ui.same_line();
                    if ui.button(">") {
                        new_duration_days = Some(task.duration + TaskDuration { days: 1, fraction: 0 });
                    }
                    ui.same_line();
                    if ui.button(">>") {
                        new_duration_days = Some(task.duration + TaskDuration { days: 7, fraction: 0 });
                    }
                    if new_duration_days.is_some() {
                        println!("New duration days: {:?}", new_duration_days);
                    }
                    if let Some(new_duration_days) = new_duration_days {
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UpdateTask {
                            id: *task_id,
                            ticket: task.ticket.clone(),
                            title: task.title.clone(),
                            duration: new_duration_days,
                        }}, self.get_timestamp().date_naive()).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                }
            }
        }
    }
}