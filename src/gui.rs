use crate::app::*;
use crate::support;
use imgui::sys::igGetCursorScreenPos;
use imgui::sys::igGetTextLineHeight;
use imgui::sys::ImVec2;
use imgui::*;
use chrono::{Utc, Datelike, NaiveDate};

pub struct Gui {
    project: Project,

    team_input_text_buffer: String,
    resource_input_text_buffer: String,
    ticket_input_text_buffer: String,
    task_title_input_text_buffer: String,
    task_duration_days: f32,
    pto_duration_days: f32,
    milestone_input_text_buffer: String,
    milestone_date_input_text_buffer: String,
}

impl Gui {
    pub fn new() -> Self {
        Gui {
            project: Project::load_from_yaml("database.yaml").unwrap_or_else(|e| {
                eprintln!("Failed to load project: {e}");
                Project::new()
            }),
            team_input_text_buffer: String::new(),
            resource_input_text_buffer: String::new(),
            ticket_input_text_buffer: String::new(),
            task_title_input_text_buffer: String::new(),
            task_duration_days: 1.0,
            pto_duration_days: 0.0,
            milestone_input_text_buffer: String::new(),
            milestone_date_input_text_buffer: String::new(),
        }
    }

    pub fn run(mut self) {
        support::simple_init(file!(), move |_run, ui| {
            unsafe {imgui::sys::igStyleColorsLight(std::ptr::null_mut());}
            
            let display_size = ui.io().display_size;
            
            if let Some(window) = ui.window("FlowState")
                .position([0.0, 0.0], Condition::Always)
                .size(display_size, Condition::Always)
                .title_bar(false)
                .resizable(false)
                .movable(false)
                .scroll_bar(false)
                .collapsible(false)
                .bring_to_front_on_focus(false)
                .nav_focus(false)
                .menu_bar(true)
                .begin()
            {
                self.draw(ui);
                window.end();
            }
        });
    }

    fn draw(&mut self, ui: &Ui) {
        self.draw_menu_bar(ui);
        self.draw_ribbon(ui);
        self.draw_tab_bar(ui);
    }

    fn draw_menu_bar(&mut self, ui: &Ui) {
        if ui.is_key_pressed(Key::Z) && ui.io().key_ctrl {
            self.project.undo().unwrap_or_else(|e| {
                eprintln!("Failed to undo: {e}");
            });
        }
        if ui.is_key_pressed(Key::Y) && ui.io().key_ctrl {
            self.project.redo().unwrap_or_else(|e| {
                eprintln!("Failed to redo: {e}");
            });
        }
        if let Some(menu_bar) = ui.begin_menu_bar() {
            if let Some(_file_menu) = ui.begin_menu("File") {
                if ui.menu_item("New Project...") {

                }
                if ui.menu_item("Open Project...") {

                }
                if ui.menu_item("Save Project") {

                }
                if ui.menu_item("Exit") {

                }
            };
            if let Some(_edit_menu) = ui.begin_menu("Edit") {
                if ui.menu_item_config("Undo").shortcut("Ctrl+Z").build() {
                    // todo!("disable undo if no commands to undo")
                    self.project.undo().unwrap_or_else(|e| {
                        eprintln!("Failed to undo: {e}");
                    });
                }
                if ui.menu_item_config("Redo").shortcut("Ctrl+Y").build() {
                    // todo!("disable redo if no commands to redo")
                    self.project.redo().unwrap_or_else(|e| {
                        eprintln!("Failed to redo: {e}");
                    });
                }
            };
            if let Some(_action_menu) = ui.begin_menu("Insert") {
                if let Some(_team_menu) = ui.begin_menu("Team") {
                    if let Some(child_window) = ui.child_window("##team_menu")
                            .size([140.0, 20.0])
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
                            self.project.invoke_command(Command::CreateTeam {
                                timestamp: Utc::now(),
                                name: self.team_input_text_buffer.clone(),
                            }).unwrap();
                            self.team_input_text_buffer.clear();
                        }
                        child_window.end();
                    }
                }
                if let Some(_milestone_menu) = ui.begin_menu("Milestone") {
                    if let Some(child_window) = ui.child_window("##team_menu")
                            .size([240.0, 50.0])
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
                            //self.flow_state.create_milestone(self.milestone_input_text_buffer.clone()).unwrap();
                            self.milestone_input_text_buffer.clear();
                        }
                        child_window.end();
                    }
                }
            };
            if let Some(_filter_menu) = ui.begin_menu("Filter") {
            };
            if let Some(_help_menu) = ui.begin_menu("Help") {
                if ui.menu_item("About") {

                }
            };
            menu_bar.end();
        };
    }

    fn draw_ribbon(&mut self, ui: &Ui) {

    }

    fn draw_tab_bar(&mut self, ui: &Ui) {
        if let Some(_tab_bar) = ui.tab_bar("##tab_bar") {
            if let Some(_res_tab_item) = ui.tab_item("Resources"){
                self.draw_gantt_chart_resources(ui);
            }
            if let Some(_task_tab_item) = ui.tab_item("Tasks") {
                self.draw_gantt_chart_tasks(ui);
            }
        }
    }

    fn draw_gantt_chart_table(&mut self, ui: &Ui, id: &str) -> bool {
        let table_id = std::ffi::CString::new(id).unwrap();
        let flags = imgui::sys::ImGuiTableFlags_Borders
            | imgui::sys::ImGuiTableFlags_HighlightHoveredColumn
            | imgui::sys::ImGuiTableFlags_SizingFixedFit
            | imgui::sys::ImGuiTableFlags_ScrollX
            | imgui::sys::ImGuiTableFlags_ScrollY
            | imgui::sys::ImGuiTableFlags_Resizable
            | imgui::sys::ImGuiTableFlags_NoPadOuterX
            | imgui::sys::ImGuiTableFlags_NoPadInnerX;
        let num_columns = self.project.flow_state().cache().num_days() + 1;
        unsafe {imgui::sys::igBeginTable(
            table_id.as_ptr(),
            num_columns as i32,
            flags as i32,
            imgui::sys::ImVec2 { x: 0.0, y: 0.0 },
            0.0,
        )}
    }

    fn draw_gantt_chart_resources(&mut self, ui: &Ui) {
        if self.draw_gantt_chart_table(ui, "##resources_gantt_chart") {
            self.draw_gantt_chart_calendar_row(ui);
            self.draw_gantt_chart_milestones_row(ui);
            self.draw_gantt_chart_resources_contents(ui);
            unsafe {imgui::sys::igEndTable();}
        }
    }

    fn draw_gantt_chart_tasks(&mut self, ui: &Ui) {
        if self.draw_gantt_chart_table(ui, "##tasks_gantt_chart") {
            self.draw_gantt_chart_calendar_row(ui);
            self.draw_gantt_chart_milestones_row(ui);
            self.draw_gantt_chart_tasks_contents(ui);
            unsafe {imgui::sys::igEndTable();}
        }
    }

    fn draw_gantt_chart_calendar_row(&mut self, ui: &Ui) {
        let mut table_data = TableColumnSetup::new("Calendar");
        table_data.flags = TableColumnFlags::NO_HIDE | TableColumnFlags::NO_REORDER;
        ui.table_setup_column_with(table_data);
        for i in 0..self.project.flow_state().cache().num_days() {
            let day: chrono::NaiveDate = self.project.flow_state().cache().day(i);
            let day_str = day.format("%m/%d").to_string();
            let day_cstr = std::ffi::CString::new(day_str).unwrap();
            unsafe {imgui::sys::igTableSetupColumn(
                day_cstr.as_ptr(),
                (imgui::sys::ImGuiTableColumnFlags_AngledHeader
                    | imgui::sys::ImGuiTableColumnFlags_WidthFixed
                    | imgui::sys::ImGuiTableColumnFlags_NoResize) as i32,
                0.0,
                0,
            );}
        }
        unsafe {imgui::sys::igTableSetupScrollFreeze(1, 4);}
        unsafe {imgui::sys::igTableAngledHeadersRow();}
        ui.table_headers_row();

        let today = chrono::Local::now().date_naive();
        for i in 0..self.project.flow_state().cache().num_days() {
            let day: chrono::NaiveDate = self.project.flow_state().cache().day(i);
            if day == today {
                let pink = [1.0, 0.75, 0.8, 1.0];
                ui.table_set_bg_color_with_column(TableBgTarget::CELL_BG, pink, i + 1);
            }
        }
    }

    fn draw_gantt_chart_milestones_row(&mut self, ui: &Ui) {
        ui.table_next_row();
        ui.table_next_column();
        ui.text("  Milestones");
        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let _id = ui.push_id_usize(i);
                let day = self.project.flow_state().cache().day(i - 1);
                if day.weekday() == chrono::Weekday::Sat || day.weekday() == chrono::Weekday::Sun {
                    let bg_color = ui.style_color(StyleColor::TableHeaderBg);
                    ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
                }
            }
        }
        ui.table_next_row();
        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let bg_color = ui.style_color(StyleColor::TableHeaderBg);
                ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
            }
        }

    }

    fn draw_gantt_chart_resources_contents(&mut self, ui: &Ui) {
        let team_ids: Vec<TeamId> = self.project.flow_state().teams.iter()
            .map(|team| team.0.clone()).collect();
        for team_id in team_ids.iter() {
            self.draw_gantt_chart_resources_team(ui, team_id);
        }
    }

    fn draw_gantt_chart_resources_team(&mut self, ui: &Ui, team_id: &TeamId) {
        ui.table_next_row();
        ui.table_next_column();

        let team = self.project.flow_state().teams.get(team_id).unwrap().clone();
        let team_name_cstr = std::ffi::CString::new(team.name.clone()).unwrap();
        let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_DefaultOpen;
        let expand_team = unsafe {
            imgui::sys::igTreeNodeEx_Str(team_name_cstr.as_ptr(), flags as i32)
        };
        self.draw_gantt_chart_resources_team_popup(ui, team_id, &team);

        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let _id = ui.push_id_usize(i);
                let day = self.project.flow_state().cache().day(i - 1);
                if day.weekday() == chrono::Weekday::Sat || day.weekday() == chrono::Weekday::Sun {
                    let bg_color = ui.style_color(StyleColor::TableHeaderBg);
                    ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
                }
            }
        }

        if expand_team {
            let resources: Vec<ResourceId> = self.project.flow_state().teams[team_id]
                .resources.iter().map(|r| r.clone()).collect();
            for resource_id in resources.iter() {
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
            imgui::sys::igTreeNodeEx_Str(resource_name_cstr.as_ptr(), flags as i32)
        };
        self.draw_gantt_chart_resources_team_resource_popup(ui, resource_id, &resource);

        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let _id = ui.push_id_usize(i);
                let day = self.project.flow_state().cache().day(i - 1);
                self.draw_weekend(ui, &day);
                self.draw_absence(ui, &day, resource_id, &resource);
                self.draw_milestone(ui, &day);
                ui.invisible_button("##invisible_button", [-1.0, unsafe { igGetTextLineHeight() }]);
                self.draw_gantt_chart_resources_team_resource_content_popup(ui, resource_id, &resource, &day);
            }
        }
        if expand_resource {
            for task_id in resource.assigned_tasks.iter() {
                self.draw_gantt_chart_resources_team_resource_task(ui, resource_id, &resource, task_id);
            }
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_gantt_chart_resources_team_resource_task(&mut self, ui: &Ui, resource_id: &ResourceId, resource: &Resource, task_id: &TaskId) {
        ui.table_next_row();
        ui.table_next_column();
        let _task_token_id = ui.push_id_int(*task_id as i32);
        let task = self.project.flow_state().tasks.get(task_id).unwrap().clone();
        let task_title_cstr = std::ffi::CString::new(format!("{} ({})", task.title, task.ticket)).unwrap();
        let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_Bullet;
        let expand_task = unsafe {
            imgui::sys::igTreeNodeEx_Str(task_title_cstr.as_ptr(), flags as i32)
        };

        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let _day_token_id = ui.push_id_usize(i);
                let day = self.project.flow_state().cache().day(i - 1);
                self.draw_weekend(ui, &day);
                self.draw_alloc(ui, &day, resource_id, &resource, task_id, &task);
                self.draw_milestone(ui, &day);
                ui.invisible_button("##invisible_button", [-1.0, unsafe { igGetTextLineHeight() }]);
            }
        }
        if expand_task {
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_gantt_chart_tasks_contents(&mut self, ui: &Ui) {
    
    }

    fn draw_weekend(&mut self, ui: &Ui, day: &NaiveDate) {
        if day.weekday() == chrono::Weekday::Sat || day.weekday() == chrono::Weekday::Sun {
            let bg_color = ui.style_color(StyleColor::TableHeaderBg);
            ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
        }
    }

    fn draw_absence(&mut self, ui: &Ui, day: &NaiveDate, resource_id: &ResourceId, resource: &Resource) {
        if let Some(absence) = self.project.flow_state().cache().resource_absence_rendering.get(resource_id).and_then(|r| r.get(day)) {
            let cell_height = unsafe { igGetTextLineHeight() };
            let cell_width = ui.current_column_width();
            let cursor_pos = unsafe {
                let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                igGetCursorScreenPos(&mut pos);
                pos
            };

            // Calculate the height of the absence rectangle based on percentage
            let absence_height = (cell_height * (*absence as f32 / 100.0)).max(1.0);

            // Draw the absence rectangle from the top of the cell
            let draw_list = ui.get_window_draw_list();
            let top_left = [cursor_pos.x, cursor_pos.y];
            let bottom_right = [cursor_pos.x + cell_width, cursor_pos.y + absence_height];
            let absence_color = [0.0, 0.0, 0.0, 1.0];

            draw_list.add_rect(top_left, bottom_right, absence_color)
                .filled(true)
                .build();
        }
    }

    fn draw_alloc(&mut self, ui: &Ui, day: &NaiveDate, resource_id: &ResourceId, resource: &Resource, task_id: &TaskId, task: &Task) {
        if let Some(alloc) = self.project.flow_state().cache().task_alloc_rendering.get(task_id)
            .and_then(|r| r.get(resource_id))
            .and_then(|r| r.get(day)) {
            let cell_height = unsafe { igGetTextLineHeight() };
            let cell_width = ui.current_column_width();
            let cursor_pos = unsafe {
                let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                igGetCursorScreenPos(&mut pos);
                pos
            };

            // Calculate the height of the allocation rectangle based on percentage
            let alloc_height = (cell_height * (*alloc as f32 / 100.0)).max(1.0);

            // Draw the allocation rectangle from the bottom of the cell
            let draw_list = ui.get_window_draw_list();
            let top_left = [cursor_pos.x, cursor_pos.y + cell_height - alloc_height];
            let bottom_right = [cursor_pos.x + cell_width, cursor_pos.y + cell_height];
            let alloc_color = [1.0, 1.0, 1.0, 1.0];

            draw_list.add_rect(top_left, bottom_right, alloc_color)
                .filled(true)
                .build();
        }
    }

    fn draw_milestone(&mut self, ui: &Ui, day: &NaiveDate) {

    }

    fn draw_gantt_chart_resources_team_popup(&mut self, ui: &Ui, team_id: &TeamId, team: &Team) {
        if let Some(popup) = ui.begin_popup_context_item() {
            if let Some(_rename_team_menu) = ui.begin_menu("Rename Team") {
                if let Some(child_window) = ui.child_window("##rename_team_menu")
                        .size([140.0, 20.0])
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
                        self.project.invoke_command(Command::RenameTeam {
                            timestamp: Utc::now(),
                            old_name: team.name.clone(),
                            new_name: self.team_input_text_buffer.clone(),
                        }).unwrap_or_else(|e| {
                            eprintln!("Failed to rename team: {e}");
                        });
                        self.team_input_text_buffer.clear();
                    }
                    child_window.end();
                }
            }
            if ui.menu_item("Delete Team") {
                self.project.invoke_command(Command::DeleteTeam {
                    timestamp: Utc::now(),
                    name: team.name.clone(),
                }).unwrap_or_else(|e| {
                    eprintln!("Failed to delete team: {e}");
                });
            }
            ui.separator();
            if let Some(_create_resource_menu) = ui.begin_menu("Create Resource") {
                if let Some(child_window) = ui.child_window("##create_resource_menu")
                        .size([140.0, 20.0])
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
                        self.project.invoke_command(Command::CreateResource {
                            timestamp: Utc::now(),
                            name: self.resource_input_text_buffer.clone(),
                            team_name: team.name.clone(),
                        }).unwrap_or_else(|e| {
                            eprintln!("Failed to create resource: {e}");
                        });
                        self.resource_input_text_buffer.clear();
                    }
                    child_window.end();
                }
            }
            popup.end();
        }
    }

    fn draw_gantt_chart_resources_team_resource_popup(&mut self, ui: &Ui, resource_id: &ResourceId, resource: &Resource) {
        let is_info_filled_in =
                |task_title: &str, ticket: &str, duration: f32| {
            !task_title.is_empty() && !ticket.is_empty() && duration > 0.0
        };
        if let Some(popup) = ui.begin_popup_context_item() {
            if let Some(_create_task_menu) = ui.begin_menu("Create Task") {
                if let Some(child_window) = ui.child_window("##create_task_menu")
                        .size([150.0, 120.0])
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
                        .display_format("%.0f days")
                        .build(&mut self.task_duration_days);
                    ui.input_float("##duration_input", &mut self.task_duration_days)
                        .display_format("%.0f days")
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
                        match self.project.invoke_command(Command::CreateTask {
                            timestamp: Utc::now(),
                            id: task_id,
                            ticket: self.ticket_input_text_buffer.clone(),
                            title: self.task_title_input_text_buffer.clone(),
                            duration: TaskDuration {
                                days: self.task_duration_days as u64,
                                fraction: (self.task_duration_days.fract() * 100.0) as u8,
                            },
                        }) {
                            Err(e) => {
                                eprintln!("Failed to create task: {e}");
                            }
                            Ok(_) => {
                                self.project.invoke_command(Command::AssignTask {
                                    timestamp: Utc::now(),
                                    task_id: task_id,
                                    resource_name: resource.name.clone(),
                                }).unwrap_or_else(|e| {
                                    eprintln!("Failed to assign task: {e}");
                                });
                            }
                        }
                        self.task_title_input_text_buffer.clear();
                    }
                    child_window.end();
                }
            }
            ui.separator();
            if let Some(_rename_resource_menu) = ui.begin_menu("Rename Resource") {
                if let Some(child_window) = ui.child_window("##rename_resource_menu")
                        .size([140.0, 20.0])
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
                        self.project.invoke_command(Command::RenameResource {
                            timestamp: Utc::now(),
                            old_name: resource.name.clone(),
                            new_name: self.resource_input_text_buffer.clone(),
                        }).unwrap_or_else(|e| {
                            eprintln!("Failed to rename resource: {e}");
                        });
                        self.resource_input_text_buffer.clear();
                    }
                    child_window.end();
                }
            }
            if ui.menu_item("Delete Resource") {
                self.project.invoke_command(Command::DeleteResource {
                    timestamp: Utc::now(),
                    name: resource.name.clone(),
                }).unwrap_or_else(|e| {
                    eprintln!("Failed to delete resource: {e}");
                });
            }
            popup.end();
        }
    }

    fn draw_gantt_chart_resources_team_resource_content_popup(&mut self, ui: &Ui, resource_id: &ResourceId, resource: &Resource, day: &NaiveDate) {
        let is_info_filled_in =
                |duration: f32| {
            duration > 0.0
        };
        let add_or_update_pto_string;
        let mut show_remove_option = false;
        if let Some(popup) = ui.begin_popup_context_item() {
            if self.project.flow_state().cache().resource_absence_rendering.get(resource_id).is_none() ||
                    self.project.flow_state().cache().resource_absence_rendering.get(resource_id).unwrap().get(day).is_none() ||
                    *self.project.flow_state().cache().resource_absence_rendering.get(resource_id).unwrap().get(day).unwrap() == 0 {
                add_or_update_pto_string = "Add PTO";
            } else {
                add_or_update_pto_string = "Update PTO";
                show_remove_option = true;
            }
            if let Some(_create_resource_menu) = ui.begin_menu(add_or_update_pto_string) {
                if let Some(child_window) = ui.child_window("##add_or_update_pto_menu")
                        .size([140.0, 20.0])
                        .begin() {
                    let mut can_add_or_update_pto = false;
                    ui.slider_config("##duration", 0.1, 30.0)
                        .display_format("%.1f days")
                        .build(&mut self.pto_duration_days);
                    ui.same_line();
                    if ui.button("Ok") {
                        can_add_or_update_pto = is_info_filled_in(self.pto_duration_days);
                    }
                    if can_add_or_update_pto {
                        ui.close_current_popup();
                        let pto_duration = TaskDuration {
                            days: self.pto_duration_days as u64,
                            fraction: (self.pto_duration_days.fract() * 100.0) as u8,
                        };
                        self.project.invoke_command(Command::SetAbsence {
                            timestamp: Utc::now(),
                            resource_name: resource.name.clone(),
                            start_date: *day,
                            days: pto_duration,
                        }).unwrap_or_else(|e| {
                            eprintln!("Failed to add PTO: {e}");
                        });
                        self.pto_duration_days = 0.0;
                    }
                    child_window.end();
                }
            }
            if show_remove_option {
                if ui.menu_item("Remove PTO") {
                    self.project.invoke_command(Command::SetAbsence {
                        timestamp: Utc::now(),
                        resource_name: resource.name.clone(),
                        start_date: *day,
                        days: TaskDuration {
                            days: 0,
                            fraction: 0,
                        },
                    }).unwrap_or_else(|e| {
                        eprintln!("Failed to remove PTO: {e}");
                    });
                }
            }
            popup.end();
        }
    }

    fn draw_gantt_chart_resources_team_resource_task_popup(&mut self, ui: &Ui, task_id: &TaskId, task: &Task) {
        // if let Some(popup) = ui.begin_popup_context_item() {
        //     if let Some(_rename_task_menu) = ui.begin_menu("Rename Task") {
        //         if let Some(child_window) = ui.child_window("##rename_task_menu")
        //                 .size([140.0, 20.0])
        //                 .begin() {
        //             let mut can_rename_task = false;
        //             if ui.input_text("##new_task_name", &mut self.task_input_text_buffer)
        //                     .enter_returns_true(true)
        //                     .hint(task.name.clone())
        //                     .build() {
        //                 can_rename_task = !self.task_input_text_buffer.is_empty();
        //             }
        //             ui.same_line();
        //             if ui.button("Ok") {
        //                 can_rename_task = !self.task_input_text_buffer.is_empty();
        //             }
        //             if can_rename_task {
        //                 ui.close_current_popup();
        //                 self.project.invoke_command(Command::RenameTask {
        //                     timestamp: Utc::now(),
        //                     old_name: task.name.clone(),
        //                     new_name: self.task_input_text_buffer.clone(),
        //                 }).unwrap_or_else(|e| {
        //                     eprintln!("Failed to rename task: {e}");
        //                 });
        //                 self.task_input_text_buffer.clear();
        //             }
        //             child_window.end();
        //         }
        //     }
        //     if ui.menu_item("Delete Task") {
        //         self.project.invoke_command(Command::DeleteTask {
        //             timestamp: Utc::now(),
        //             id: task.id,
        //         }).unwrap_or_else(|e| {
        //             eprintln!("Failed to delete task: {e}");
        //         });
        //     }
        //     popup.end();
        // }
    }
    
    fn draw_gantt_chart_resources_team_resource_task_content_popup(&mut self, ui: &Ui, task_id: &TaskId, task: &Task) {
        // if let Some(popup) = ui.begin_popup_context_item() {
        //     if let Some(_edit_task_menu) = ui.begin_menu("Edit Task") {
        //         if let Some(child_window) = ui.child_window("##edit_task_menu")
        //                 .size([140.0, 20.0])
        //                 .begin() {
        //             let mut can_edit_task = false;
        //             if ui.input_text("##edit_task_name", &mut self.task_input_text_buffer)
        //                     .enter_returns_true(true)
        //                     .hint(task.name.clone())
        //                     .build() {
        //                 can_edit_task = !self.task_input_text_buffer.is_empty();
        //             }
        //             ui.same_line();
        //             if ui.button("Ok") {
        //                 can_edit_task = !self.task_input_text_buffer.is_empty();
        //             }
        //             if can_edit_task {
        //                 ui.close_current_popup();
        //                 self.project.invoke_command(Command::EditTask {
        //                     timestamp: Utc::now(),
        //                     id: *task_id,
        //                     new_name: self.task_input_text_buffer.clone(),
        //                 }).unwrap_or_else(|e| {
        //                     eprintln!("Failed to edit task: {e}");
        //                 });
        //                 self.task_input_text_buffer.clear();
        //             }
        //             child_window.end();
        //         }
        //     }
        //     if ui.menu_item("Delete Task") {
        //         self.project.invoke_command(Command::DeleteTask {
        //             timestamp: Utc::now(),
        //             id: task.id,
        //         }).unwrap_or_else(|e| {
        //             eprintln!("Failed to delete task: {e}");
        //         });
        //     }
        //     popup.end();
        // }
    }
}


