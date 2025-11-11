
use crate::app::*;
use crate::gui;
use crate::support;
use imgui::sys::igGetCursorScreenPos;
use imgui::sys::igGetTextLineHeight;
use imgui::sys::ImVec2;
use imgui::*;
use chrono::{Utc, DateTime, Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const NEW_PROJECT_CHILD_WINDOW_SIZE: [f32; 2] = [240.0, 30.0];
const CREATE_TEAM_CHILD_WINDOW_SIZE: [f32; 2] = NEW_PROJECT_CHILD_WINDOW_SIZE;
const RENAME_TEAM_CHILD_WINDOW_SIZE: [f32; 2] = CREATE_TEAM_CHILD_WINDOW_SIZE;
const CREATE_RESOURCE_CHILD_WINDOW_SIZE: [f32; 2] = CREATE_TEAM_CHILD_WINDOW_SIZE;
const RENAME_RESOURCE_CHILD_WINDOW_SIZE: [f32; 2] = CREATE_RESOURCE_CHILD_WINDOW_SIZE;
const CREATE_TASK_CHILD_WINDOW_SIZE: [f32; 2] = [180.0, 150.0];
const UPDATE_TASK_CHILD_WINDOW_SIZE: [f32; 2] = CREATE_TASK_CHILD_WINDOW_SIZE;
const CREATE_MILESTONE_CHILD_WINDOW_SIZE: [f32; 2] = [240.0, 70.0];
const SET_WORKLOG_CHILD_WINDOW_SIZE: [f32; 2] = [260.0, 100.0];
const CREATE_LABEL_CHILD_WINDOW_SIZE: [f32; 2] = [240.0, 70.0];

pub struct Gui {
    gui_config: GuiConfig,
    project: Project,

    filtered_labels: Vec<LabelId>,
    selected_filter: Option<FilterId>,

    drag_drop_task_id: Option<TaskId>,
    date_offset: i32,

    bold_font: std::rc::Rc<std::cell::RefCell<Option<FontId>>>,
    find_input_buffer: String,
    new_project_input_text_buffer: String,
    team_input_text_buffer: String,
    resource_input_text_buffer: String,
    ticket_input_text_buffer: String,
    task_title_input_text_buffer: String,
    task_duration_days: f32,
    absence_duration_days: f32,
    worklog_fraction: u8,
    milestone_input_text_buffer: String,
    milestone_date_input_text_buffer: String,
    label_input_text_buffer: String,
    filter_input_text_buffer: String,
    logs: Vec<String>,
    drawing_aids: DrawingAids,
}

struct DrawingAids {
    previous_rect: Option<(ImVec2, ImVec2)>,
    row_counter: usize,
    pending_draws: Vec<([f32; 2], [f32; 4], String)>,
}

impl DrawingAids {
    pub fn new() -> Self {
        DrawingAids { previous_rect: None, row_counter: 0, pending_draws: Vec::new() }
    }
}

macro_rules! gui_log {
    ($gui:expr, $($arg:tt)*) => {
        $gui.log(format!($($arg)*))
    };
}

impl Gui {
    pub fn new() -> Self {
        let gui_config = GuiConfig::load_from_yaml("config.yaml");
        let yaml_filename = gui_config.recent_project_files.first().cloned().unwrap_or_else(|| "database.yaml".to_string());
        Gui {
            gui_config,
            project: Project::load_from_yaml(&yaml_filename).unwrap_or_else(|e| {
                eprintln!("Failed to load project: {e}");
                Project::new(&yaml_filename)
            }),

            filtered_labels: Vec::new(),
            selected_filter: None,

            drag_drop_task_id: None,
            date_offset: 0,

            bold_font: std::rc::Rc::new(std::cell::RefCell::new(None)),
            find_input_buffer: String::new(),
            new_project_input_text_buffer: String::new(),
            team_input_text_buffer: String::new(),
            resource_input_text_buffer: String::new(),
            ticket_input_text_buffer: "FCA_NRTRIC-".to_string(),
            task_title_input_text_buffer: String::new(),
            task_duration_days: 1.0,
            absence_duration_days: 0.0,
            worklog_fraction: 0,
            milestone_input_text_buffer: String::new(),
            milestone_date_input_text_buffer: String::new(),
            label_input_text_buffer: String::new(),
            filter_input_text_buffer: String::new(),
            logs: Vec::new(),
            drawing_aids: DrawingAids::new(),
        }
    }

    fn log(&mut self, message: String) {
        self.logs.push(message);
        if self.logs.len() > 10 {
            self.logs.drain(0..self.logs.len() - 10);
        }
    }
}

impl Gui {
    pub fn run(mut self) {
        let bold_font_for_init = self.bold_font.clone();
        support::init_with_startup(
            file!(),
            move |ctx, renderer, _| {
                ctx.set_ini_filename(Some(std::path::PathBuf::from("imgui.ini")));
                let mut bold_font_handle = bold_font_for_init.borrow_mut();
                *bold_font_handle = Some(ctx.fonts().add_font(&[FontSource::TtfData {
                    data: include_bytes!("../res/Roboto-Bold.ttf"),
                    size_pixels: support::FONT_SIZE,
                    config: None,
                }]));
                renderer
                    .reload_font_texture(ctx)
                    .expect("Failed to reload fonts");
            },
            move |_run, ui| {
                unsafe {imgui::sys::igStyleColorsLight(std::ptr::null_mut());}

                let display_size = ui.io().display_size;
                
                if let Some(_window) = ui.window("FlowState")
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
                }
            }
        );
    }

    fn draw(&mut self, ui: &Ui) {
        self.draw_menu_bar(ui);
        self.draw_ribbon(ui);
        self.draw_tab_bar(ui);
        self.apply_pending_draws(ui);
    }

    fn draw_menu_bar(&mut self, ui: &Ui) {
        if ui.is_key_pressed(Key::Z) && ui.io().key_ctrl {
            self.project.undo().unwrap_or_else(|e| {
                gui_log!(self, "Failed to undo: {e}");
            });
        }
        if ui.is_key_pressed(Key::Y) && ui.io().key_ctrl {
            self.project.redo().unwrap_or_else(|e| {
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
                        match Project::load_from_yaml(&file_path_str) {
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
                            match Project::load_from_yaml(recent_file) {
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
                    self.project.undo().unwrap_or_else(|e| {
                        gui_log!(self, "Failed to undo: {e}");
                    });
                }
                if ui.menu_item_config("Redo").shortcut("Ctrl+Y").build() {
                    self.project.redo().unwrap_or_else(|e| {
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
                            }}).unwrap();
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
                                }}).unwrap();
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
                                }}).unwrap();
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
                            }}).unwrap();
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
                        }}).unwrap();
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
                            }}).unwrap();
                            if is_selected {
                                self.selected_filter = None;
                                self.filtered_labels.clear();
                            }
                        }
                    }
                }
                if let Some(_favorite_filter_menu) = ui.begin_menu("Favorites") {
                    for (filter_id, filter) in &filters {
                        let is_favorite = filter.is_favorite;
                        if ui.menu_item_config(&filter.name).selected(is_favorite).build() {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::CreateModifyFilter {
                                name: filter.name.clone(),
                                labels: filter.labels.iter()
                                    .filter_map(|&label_id| self.project.flow_state().labels.get(&label_id))
                                    .map(|label| label.name.clone())
                                    .collect(),
                                is_favorite: !is_favorite,
                            }}).unwrap();
                        }
                    }
                }
            }
            if let Some(_filters_menu) = ui.begin_menu("View") {
                if ui.menu_item_config("Hide Worklogs").selected(self.gui_config.hide_worklogs).build() {
                    self.gui_config.hide_worklogs = !self.gui_config.hide_worklogs;
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

    fn draw_ribbon(&mut self, ui: &Ui) {
        ui.align_text_to_frame_padding();
        ui.text("Find");
        ui.same_line();
        ui.set_next_item_width(200.0);
        
        if ui.is_key_pressed(Key::F) && ui.io().key_ctrl {
            ui.set_keyboard_focus_here();
        }
        ui.input_text("##find", &mut self.find_input_buffer).build();
        
        if ui.is_key_pressed(Key::Escape) {
            self.find_input_buffer.clear();
        }

        if self.gui_config.debug_mode {
            ui.same_line();
            ui.set_next_item_width(80.0);
            ui.input_int("##date_offset_input", &mut self.date_offset)
                        .step(1)
                        .build();
        }

        for (filter_id, filter) in &self.project.flow_state().filters {
            if filter.is_favorite {
                ui.same_line();
                if ui.radio_button(&filter.name, &mut self.selected_filter, Some(*filter_id)) {
                    self.selected_filter = Some(*filter_id);
                    self.filtered_labels = filter.labels.iter().cloned().collect();
                }
            }
        }
    }

    fn draw_tab_bar(&mut self, ui: &Ui) {
        if let Some(_tab_bar) = ui.tab_bar("##tab_bar") {
            if let Some(_res_tab_item) = ui.tab_item("Resources"){
                self.draw_gantt_chart_resources(ui);
            }
            if let Some(_task_tab_item) = ui.tab_item("Tasks") {
                self.draw_gantt_chart_tasks(ui);
            }
            if let Some(_debug_tab_item) = ui.tab_item("Debug") {
                self.draw_debug(ui);
            }
            if let Some(_log_tab_item) = ui.tab_item("Logs") {
                if let Some(_table) = ui.begin_table_with_flags("##gui_logs_table", 1, TableFlags::BORDERS | TableFlags::ROW_BG | TableFlags::SCROLL_Y) {
                    ui.table_setup_column("GUI Log Messages");
                    ui.table_headers_row();
                    for log in self.logs.iter().rev() {
                        ui.table_next_row();
                        ui.table_next_column();
                        ui.text(log);
                    } 
                }
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

    fn draw_debug(&mut self, ui: &Ui) {
        let flow_state_str = format!("{:#?}", self.project.flow_state());
        ui.text(flow_state_str);
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

        let today = if self.date_offset < 0 {
            chrono::Local::now().date_naive() - chrono::Days::new(self.date_offset.abs() as u64)
        } else {
            chrono::Local::now().date_naive() + chrono::Days::new(self.date_offset.abs() as u64)
        };
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
                if let Some(milestones) = self.project.flow_state().cache().date_to_milestones.get(&day) {
                    for milestone in milestones {
                        let cursor_pos = ui.cursor_screen_pos();

                        let text_size = ui.calc_text_size(&milestone.title);
                        let column_width = ui.current_column_width();

                        let text_pos = [
                            cursor_pos[0] + (column_width - text_size[0]) * 0.5,
                            cursor_pos[1],
                        ];
                        let text_color = ui.style_color(StyleColor::Text);
                        self.drawing_aids.pending_draws.push((text_pos, text_color, milestone.title.clone()));
                    }
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
        if ui.drag_drop_source_config("DND_TASK").begin().is_some() {
            self.drag_drop_task_id = Some(*task_id);
        }
        if let Some(target) = ui.drag_drop_target() {
            if target
                .accept_payload_empty("DND_TASK", DragDropFlags::empty())
                .is_some()
            {
                let msg = self.drag_drop_task_id.unwrap();
                println!("Dropped task {msg} on task {}", *task_id);
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
                self.draw_alloc(ui, &day, Some(resource_id), task_id, &task);
                self.draw_milestone(ui, &day);
                ui.invisible_button("##invisible_button", [-1.0, unsafe { igGetTextLineHeight() }]);
                self.draw_gantt_chart_resources_team_resource_task_content_popup(ui, resource_id, &resource, task_id, &task, &day);
                if ui.is_item_hovered() {
                    let absence = self.project.flow_state().cache().resource_absence_rendering.get(resource_id)
                            .and_then(|r| r.get(&day));
                    let worklog = self.project.flow_state().worklogs.get(&task_id)
                            .and_then(|r| r.get(&resource_id))
                            .and_then(|r| r.get(&day));
                    let alloc = self.project.flow_state().cache().task_alloc_rendering.get(task_id)
                            .and_then(|r| r.get(&resource_id))
                            .and_then(|r| r.get(&day));
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
                self.draw_alloc(ui, &day, None, task_id, &task);
                self.draw_milestone(ui, &day);
                ui.invisible_button("##invisible_button", [-1.0, unsafe { igGetTextLineHeight() }]);
                self.draw_gantt_chart_resources_team_unassigned_task_content_popup(ui, task_id, &task, &day);
            }
        }
        if expand_task {
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_gantt_chart_tasks_contents(&mut self, ui: &Ui) {
        let task_ids: Vec<TaskId> = self.project.flow_state().tasks.keys().cloned().collect();
        for (i, task_id) in task_ids.iter().enumerate() {
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
                self.drawing_aids.row_counter = i;
                self.draw_gantt_chart_tasks_task(ui, task_id, &task);
            }
        }

    }

    fn draw_cell_background(&mut self, ui: &Ui, day: &NaiveDate) {
        if day.weekday() == chrono::Weekday::Sat || day.weekday() == chrono::Weekday::Sun {
            let bg_color = ui.style_color(StyleColor::TableHeaderBg);
            ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
        } else {
            let bg_color = if self.drawing_aids.row_counter % 2 == 0 {
                ui.style_color(StyleColor::TableRowBg)
            } else {
                ui.style_color(StyleColor::TableRowBgAlt)
            };
            ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
        }
    }

    fn draw_absence(&mut self, ui: &Ui, day: &NaiveDate, resource_id: &ResourceId, resource: &Resource) {
        if let Some(absence) = self.project.flow_state().cache().resource_absence_rendering.get(resource_id)
                .and_then(|r| r.get(day)) {
            let cell_height = unsafe { igGetTextLineHeight() };
            let cell_padding = unsafe { ui.style().cell_padding };
            let effective_cell_height = cell_height + 1.5 * cell_padding[1];
            let effective_cell_width = ui.current_column_width();

            let cursor_pos = unsafe {
                let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                igGetCursorScreenPos(&mut pos);
                pos.y -= cell_padding[1] / 2.0;
                pos
            };

            let absence_height = (effective_cell_height * (*absence as f32 / 100.0)).max(1.0);
            let draw_list = ui.get_window_draw_list();
            let top_left = [cursor_pos.x, cursor_pos.y];
            let bottom_right = [cursor_pos.x + effective_cell_width, cursor_pos.y + absence_height];
            let absence_color = [0.0, 0.0, 0.0, 1.0];
            let border_color = [0.0, 0.0, 0.0, 1.0];

            draw_list.add_rect(top_left, bottom_right, absence_color)
                .filled(true)
                .build();

            draw_list.add_rect(top_left, bottom_right, border_color)
                .thickness(1.0)
                .build();
        }
    }

    fn draw_alloc(&mut self, ui: &Ui, day: &NaiveDate, resource_id: Option<&ResourceId>, task_id: &TaskId, task: &Task) {
        let cell_height = unsafe { igGetTextLineHeight() };
        let cell_padding = unsafe { ui.style().cell_padding };
        let effective_cell_height = cell_height + (cell_padding[1]);
        let effective_cell_width = ui.current_column_width();

        let worklog = self.project.flow_state().worklogs.get(task_id)
            .and_then(|r| r.get(resource_id.unwrap_or(&0)))
            .and_then(|r| r.get(day));
        let alloc = if let Some(resource_id) = resource_id {
            self.project.flow_state().cache().task_alloc_rendering.get(task_id)
                .and_then(|r| r.get(resource_id))
                .and_then(|r| r.get(day))
        } else {
            self.project.flow_state().cache().unassigned_task_alloc_rendering.get(task_id)
                .and_then(|r| r.get(day))
        };
        if let Some(alloc) = alloc {
            let cursor_pos = unsafe {
                let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                igGetCursorScreenPos(&mut pos);
                pos.y -= cell_padding[1] / 2.0;
                pos
            };

            let alloc_height = effective_cell_height * (*alloc as f32 / 100.0);
            let worklog_height = if let Some(worklog) = worklog {
                effective_cell_height * (worklog.fraction as f32) / 100.0
            } else {
                0.0
            };

            let draw_list = ui.get_window_draw_list();
            let top_left = [cursor_pos.x, cursor_pos.y + effective_cell_height - worklog_height - alloc_height];
            let bottom_right = [cursor_pos.x + effective_cell_width, cursor_pos.y + effective_cell_height - worklog_height];
            let alloc_color = [1.0, 1.0, 1.0, 1.0];
            let border_color = [0.0, 0.0, 0.0, 1.0];

            draw_list.add_rect(top_left, bottom_right, alloc_color)
                .filled(true)
                .build();

            if let Some(prev_rect) = self.drawing_aids.previous_rect {
                let left_top = [cursor_pos.x, cursor_pos.y + effective_cell_height - worklog_height - alloc_height];
                let left_bottom = [cursor_pos.x, prev_rect.0.y];
                draw_list.add_line(left_top, left_bottom, border_color)
                    .thickness(1.0)
                    .build();
            } else {
                let left_top = [cursor_pos.x, cursor_pos.y + effective_cell_height - worklog_height - alloc_height];
                let left_bottom = [cursor_pos.x, cursor_pos.y + effective_cell_height - worklog_height];
                draw_list.add_line(left_top, left_bottom, border_color)
                    .thickness(1.0)
                    .build();
            }
            
            let top_left_border = [cursor_pos.x, cursor_pos.y + effective_cell_height - worklog_height - alloc_height];
            let top_right_border = [cursor_pos.x + effective_cell_width, cursor_pos.y + effective_cell_height  - worklog_height - alloc_height];
            let bottom_left_border = [cursor_pos.x, cursor_pos.y + effective_cell_height  - worklog_height];
            let bottom_right_border = [cursor_pos.x + effective_cell_width, cursor_pos.y + effective_cell_height  - worklog_height];

            draw_list.add_line(top_left_border, top_right_border, border_color)
                .thickness(1.0)
                .build();
            draw_list.add_line(bottom_left_border, bottom_right_border, border_color)
                .thickness(1.0)
                .build();
            self.drawing_aids.previous_rect = Some((
                ImVec2 { x: top_left[0], y: top_left[1] },
                ImVec2 { x: bottom_right[0], y: bottom_right[1] }
            ));
        } else {
            if let Some(prev_rect) = self.drawing_aids.previous_rect {
                let cursor_pos = unsafe {
                    let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                    igGetCursorScreenPos(&mut pos);
                    pos
                };
                let border_color = [0.0, 0.0, 0.0, 1.0];

                let right_top = [cursor_pos.x, prev_rect.0.y];
                let right_bottom = [cursor_pos.x, cursor_pos.y + effective_cell_height];

                ui.get_window_draw_list().add_line(right_top, right_bottom, border_color)
                    .thickness(1.0)
                    .build();
                self.drawing_aids.previous_rect = None;
            }
        }
    }

    fn draw_alloc_as_watcher(&mut self, ui: &Ui, day: &NaiveDate, resource_id: Option<&ResourceId>, task_id: &TaskId, task: &Task) {
        let cell_height = unsafe { igGetTextLineHeight() };
        let cell_padding = unsafe { ui.style().cell_padding };
        let effective_cell_height = cell_height + (cell_padding[1]);
        let effective_cell_width = ui.current_column_width();

        let worklog = self.project.flow_state().worklogs.get(task_id)
            .and_then(|r| r.get(resource_id.unwrap_or(&0)))
            .and_then(|r| r.get(day));
        let alloc = if let Some(resource_id) = resource_id {
            self.project.flow_state().cache().task_alloc_rendering.get(task_id)
                .and_then(|r| r.get(resource_id))
                .and_then(|r| r.get(day))
        } else {
            self.project.flow_state().cache().unassigned_task_alloc_rendering.get(task_id)
                .and_then(|r| r.get(day))
        };
        if let Some(alloc) = alloc {
            let cursor_pos = unsafe {
                let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                igGetCursorScreenPos(&mut pos);
                pos.y -= cell_padding[1] / 2.0;
                pos
            };

            let alloc_height = effective_cell_height * (*alloc as f32 / 100.0);
            let worklog_height = if let Some(worklog) = worklog {
                effective_cell_height * (worklog.fraction as f32) / 100.0
            } else {
                0.0
            };

            let draw_list = ui.get_window_draw_list();
            let top_left = [cursor_pos.x, cursor_pos.y + effective_cell_height - worklog_height - alloc_height];
            let bottom_right = [cursor_pos.x + effective_cell_width, cursor_pos.y + effective_cell_height - worklog_height];
            let alloc_color = [0.9, 0.9, 0.9, 1.0];
            let border_color = [0.0, 0.0, 0.0, 1.0];

            draw_list.add_rect(top_left, bottom_right, alloc_color)
                .filled(true)
                .build();

            if let Some(prev_rect) = self.drawing_aids.previous_rect {
                let left_top = [cursor_pos.x, cursor_pos.y + effective_cell_height - worklog_height - alloc_height];
                let left_bottom = [cursor_pos.x, prev_rect.0.y];
                draw_list.add_line(left_top, left_bottom, border_color)
                    .thickness(1.0)
                    .build();
            } else {
                let left_top = [cursor_pos.x, cursor_pos.y + effective_cell_height - worklog_height - alloc_height];
                let left_bottom = [cursor_pos.x, cursor_pos.y + effective_cell_height - worklog_height];
                draw_list.add_line(left_top, left_bottom, border_color)
                    .thickness(1.0)
                    .build();
            }
            
            let top_left_border = [cursor_pos.x, cursor_pos.y + effective_cell_height - worklog_height - alloc_height];
            let top_right_border = [cursor_pos.x + effective_cell_width, cursor_pos.y + effective_cell_height  - worklog_height - alloc_height];
            let bottom_left_border = [cursor_pos.x, cursor_pos.y + effective_cell_height  - worklog_height];
            let bottom_right_border = [cursor_pos.x + effective_cell_width, cursor_pos.y + effective_cell_height  - worklog_height];

            draw_list.add_line(top_left_border, top_right_border, border_color)
                .thickness(1.0)
                .build();
            draw_list.add_line(bottom_left_border, bottom_right_border, border_color)
                .thickness(1.0)
                .build();
            self.drawing_aids.previous_rect = Some((
                ImVec2 { x: top_left[0], y: top_left[1] },
                ImVec2 { x: bottom_right[0], y: bottom_right[1] }
            ));
        } else {
            if let Some(prev_rect) = self.drawing_aids.previous_rect {
                let cursor_pos = unsafe {
                    let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                    igGetCursorScreenPos(&mut pos);
                    pos
                };
                let border_color = [0.0, 0.0, 0.0, 1.0];

                let right_top = [cursor_pos.x, prev_rect.0.y];
                let right_bottom = [cursor_pos.x, cursor_pos.y + effective_cell_height];

                ui.get_window_draw_list().add_line(right_top, right_bottom, border_color)
                    .thickness(1.0)
                    .build();
                self.drawing_aids.previous_rect = None;
            }
        }
    }

    fn draw_worklog(&mut self, ui: &Ui, day: &NaiveDate, resource_id: &ResourceId, resource: &Resource, task_id: &TaskId, task: &Task) {
        let cell_height = unsafe { igGetTextLineHeight() };
        let cell_padding = unsafe { ui.style().cell_padding };
        let effective_cell_height = cell_height + 1.5 * cell_padding[1];
        let effective_cell_width = ui.current_column_width();

        let worklog = self.project.flow_state().worklogs.get(&task_id)
            .and_then(|r| r.get(&resource_id))
            .and_then(|r| r.get(day));

        if let Some(worklog) = worklog {
            let cursor_pos = unsafe {
                let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                igGetCursorScreenPos(&mut pos);
                pos.y -= cell_padding[1] / 2.0;
                pos
            };
            let worklog_height = effective_cell_height * (worklog.fraction as f32) / 100.0;
            let worklog_p1 = [
                cursor_pos.x,
                cursor_pos.y + effective_cell_height - worklog_height,
            ];
            let worklog_p2 = [
                cursor_pos.x + effective_cell_width,
                cursor_pos.y + effective_cell_height,
            ];
            ui.get_window_draw_list().add_rect(worklog_p1, worklog_p2, [0.32, 0.58, 0.83, 1.0])
                .filled(true)
                .build();
        }
    }

    fn draw_worklog_on_others_tasks(&mut self, ui: &Ui, day: &NaiveDate, resource_id: &ResourceId, resource: &Resource) {
        let cell_height = unsafe { igGetTextLineHeight() };
        let cell_padding = unsafe { ui.style().cell_padding };
        let effective_cell_height = cell_height + 1.5 * cell_padding[1];
        let effective_cell_width = ui.current_column_width();

        let worklog = self.project.flow_state().cache().worklogs_on_others_tasks.get(resource_id)
            .and_then(|r| r.get(day));

        if let Some(worklog) = worklog {
            let cursor_pos = unsafe {
                let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                igGetCursorScreenPos(&mut pos);
                pos.y -= cell_padding[1] / 2.0;
                pos
            };
            let worklog_height = effective_cell_height * (*worklog as f32) / 100.0;
            let worklog_p1 = [
                cursor_pos.x,
                cursor_pos.y + effective_cell_height - worklog_height,
            ];
            let worklog_p2 = [
                cursor_pos.x + effective_cell_width,
                cursor_pos.y + effective_cell_height,
            ];
            ui.get_window_draw_list().add_rect(worklog_p1, worklog_p2, [0.6, 0.6, 0.6, 1.0])
                .filled(true)
                .build();

            if ui.is_item_hovered() {
                if let Some(_tooltip) = ui.begin_tooltip() {
                    let absence = self.project.flow_state().cache().resource_absence_rendering.get(resource_id)
                            .and_then(|r| r.get(day));
                    if let Some(absence) = absence {
                        ui.bullet_text(format!("Absence: {}%", absence));
                    }
                    for (task_id, task) in &self.project.flow_state().tasks {
                        if task.assignee.as_ref() == Some(resource_id) {
                            continue;
                        }
                        
                        if let Some(worklog) = self.project.flow_state().worklogs.get(task_id)
                            .and_then(|task_worklogs| task_worklogs.get(resource_id))
                            .and_then(|resource_worklogs| resource_worklogs.get(day)) {
                            
                            let assignee_name = task.assignee
                                .and_then(|id| self.project.flow_state().resources.get(&id))
                                .map(|r| r.name.clone())
                                .unwrap_or_else(|| "Unassigned".to_string());
                            
                            ui.bullet_text(&format!("{} - {} ({}): {}%", 
                                task.ticket, 
                                if task.title.len() > 40 {
                                    format!("{}...", task.title.chars().take(40).collect::<String>())
                                } else {
                                    task.title.clone()
                                }, 
                                assignee_name, 
                                worklog.fraction
                            ));
                        }
                    }
                }
            }
        }
    }

    fn draw_milestone(&mut self, ui: &Ui, day: &NaiveDate) {
        let today = chrono::Local::now().date_naive();
        if let Some(milestones) = self.project.flow_state().cache().date_to_milestones.get(day) {
            let cell_height = unsafe { igGetTextLineHeight() };
            let cell_padding = unsafe { ui.style().cell_padding };
            let effective_cell_height = cell_height + (2.0 * cell_padding[1]);
            let effective_cell_width = ui.current_column_width();

            let cursor_pos = unsafe {
                let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                igGetCursorScreenPos(&mut pos);
                pos.y -= cell_padding[1] / 2.0;
                pos
            };

            let draw_list = ui.get_window_draw_list();
            
            // Create gradient from transparent red to opaque red on the right edge
            let gradient_start = [cursor_pos.x + (0.9 * effective_cell_width), cursor_pos.y];
            let gradient_end = [cursor_pos.x + effective_cell_width, cursor_pos.y + effective_cell_height];
            
            let transparent_red = [1.0, 0.0, 0.0, 0.0];
            let opaque_red = [1.0, 0.0, 0.0, 0.7];
            
            // Draw gradient rectangle for milestone indicator
            draw_list.add_rect_filled_multicolor(
                gradient_start,
                gradient_end,
                transparent_red,
                opaque_red,
                opaque_red,
                transparent_red,
            );
        }
    }

    fn draw_gantt_chart_resources_team_popup(&mut self, ui: &Ui, team_id: &TeamId, team: &Team) {
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
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to rename team: {e}");
                        });
                        self.team_input_text_buffer.clear();
                    }
                }
            }
            if ui.menu_item("Delete Team") {
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::DeleteTeam {
                    name: team.name.clone(),
                }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to create resource: {e}");
                        });
                        self.resource_input_text_buffer.clear();
                    }
                }
            }
        }
    }

    fn draw_gantt_chart_resources_team_resource_popup(&mut self, ui: &Ui, resource_id: &ResourceId, resource: &Resource) {
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
                            }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to rename resource: {e}");
                        });
                        self.resource_input_text_buffer.clear();
                    }
                }
            }
            if ui.menu_item("Delete Resource") {
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::DeleteResource {
                    name: resource.name.clone(),
                }}).unwrap_or_else(|e| {
                    eprintln!("Failed to delete resource: {e}");
                });
            }
        }
    }

    fn draw_gantt_chart_resources_team_resource_content_popup(&mut self, ui: &Ui, resource_id: &ResourceId, resource: &Resource, day: &NaiveDate) {
        let is_info_filled_in =
                |duration: f32| {
            duration > 0.0
        };
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
                        }}).unwrap_or_else(|e| {
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
                    }}).unwrap_or_else(|e| {
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
                }}).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to move task to top: {e}");
                });
            }
            if ui.menu_item("Move up") {
                ui.close_current_popup();
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::PrioritizeTask {
                    task_id: *task_id,
                    to_top: false
                }}).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to move task up: {e}");
                });
            }
            if ui.menu_item("Move down") {
                ui.close_current_popup();
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::DeprioritizeTask {
                    task_id: *task_id,
                    to_bottom: false
                }}).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to move task down: {e}");
                });
            }
            if ui.menu_item("Move to bottom") {
                ui.close_current_popup();
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::DeprioritizeTask {
                    task_id: *task_id,
                    to_bottom: true
                }}).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to move task to bottom: {e}");
                });
            }
            ui.separator();
            if let Some(_assign_to_menu) = ui.begin_menu("Assign to") {
                let mut resources: Vec<_> = self.project.flow_state().resources.values().cloned().collect();
                resources.sort_by(|a, b| a.name.cmp(&b.name));
                for resource in resources {
                    if ui.menu_item(resource.name.clone()) {
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AssignTask {
                            task_id: *task_id,
                            resource_name: resource.name,
                        }}).unwrap_or_else(|e| {
                            gui_log!(self, "Failed to assign task to resource: {e}");
                        });
                    }
                }
            }
            if ui.menu_item("Unassign") {
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::UnassignTask {
                    task_id: *task_id,
                }}).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to unassign task: {e}");
                });
                ui.close_current_popup();
            }
            if let Some(_watchers_menu) = ui.begin_menu("Watchers") {
                /* list all the resources as a menu item. If the resource is already a watcher, it should be checked */
                let mut resources: Vec<_> = self.project.flow_state().resources.values().cloned().collect();
                resources.sort_by(|a, b| a.name.cmp(&b.name));
                for resource in resources {
                    let is_watching = resource.watched_tasks.contains(task_id);
                    if ui.menu_item_config(resource.name.clone()).selected(is_watching).build() {
                        if is_watching {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to unwatch task: {e}");
                            });
                        } else {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to watch task: {e}");
                            });
                        }
                    }
                }
            }
            ui.separator();
            ui.disabled(true, || {
                ui.menu_item("Delete");
            });
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
                        }}).unwrap_or_else(|e| {
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
                            }}).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to remove label from task: {e}");
                            });
                        } else {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddLabelToTask {
                                task_id: *task_id,
                                label_name: label.name.clone(),
                            }}).unwrap_or_else(|e| {
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
                            }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                }
            }
        }
    }

    fn draw_gantt_chart_resources_team_resource_task_as_watcher_popup(&mut self, ui: &Ui, task_id: &TaskId, task: &Task, resource_id: &ResourceId, resource: &Resource) {
        if let Some(_popup) = ui.begin_popup_context_item() {
            if let Some(_assign_to_menu) = ui.begin_menu("Assign to") {
                let mut resources: Vec<_> = self.project.flow_state().resources.values().cloned().collect();
                resources.sort_by(|a, b| a.name.cmp(&b.name));
                for resource in resources {
                    if ui.menu_item(resource.name.clone()) {
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AssignTask {
                            task_id: *task_id,
                            resource_name: resource.name,
                        }}).unwrap_or_else(|e| {
                            gui_log!(self, "Failed to assign task to resource: {e}");
                        });
                    }
                }
            }
            if ui.menu_item("Unwatch") {
                self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveWatcher {
                    task_id: *task_id,
                    resource_name: resource.name.clone(),
                }}).unwrap_or_else(|e| {
                    gui_log!(self, "Failed to unwatch task: {e}");
                });
                ui.close_current_popup();
            }
            if let Some(_watchers_menu) = ui.begin_menu("Watchers") {
                /* list all the resources as a menu item. If the resource is already a watcher, it should be checked */
                let mut resources: Vec<_> = self.project.flow_state().resources.values().cloned().collect();
                resources.sort_by(|a, b| a.name.cmp(&b.name));
                for resource in resources {
                    let is_watching = resource.watched_tasks.contains(task_id);
                    if ui.menu_item_config(resource.name.clone()).selected(is_watching).build() {
                        if is_watching {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to unwatch task: {e}");
                            });
                        } else {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
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
                            }}).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to remove label from task: {e}");
                            });
                        } else {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddLabelToTask {
                                task_id: *task_id,
                                label_name: label.name.clone(),
                            }}).unwrap_or_else(|e| {
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
                            }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
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
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: self.worklog_fraction,
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    if ui.button("0%") {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 0,
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    ui.same_line();
                    if ui.button("10%") {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 10,
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    ui.same_line();
                    if ui.button("25%") {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 25,
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    ui.same_line();
                    if ui.button("50%") {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 50,
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    ui.same_line();
                    if ui.button("75%") {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 75,
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }                    
                    ui.same_line();
                    if ui.button("100%") {
                        ui.close_current_popup();
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::SetWorklog {
                            task_id: *task_id,
                            date: *day,
                            resource_name: resource.name.clone(),
                            fraction: 100,
                        }}).unwrap_or_else(|e| {
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
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::SetWorklog {
                                task_id: *task_id,
                                date: *day,
                                resource_name: resource.name.clone(),
                                fraction: current_worklog_fraction + remaining_fraction as u8,
                            }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                    if ui.button("Crop") {
                        if let Some(task_alloc_for_day) = self.project.flow_state().cache().task_alloc_rendering.get(task_id)
                                .and_then(|r| r.get(resource_id))
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
                            }}).unwrap_or_else(|e| {
                                eprintln!("Failed to crop task: {e}");
                            });
                        }
                    }
                    if ui.button("Round up") {
                        if let Some(task_alloc_for_day) = self.project.flow_state().cache().task_alloc_rendering.get(task_id)
                                .and_then(|r| r.get(resource_id))
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
                            }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
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
                resources.sort_by(|a, b| a.name.cmp(&b.name));
                for resource in resources {
                    if ui.menu_item(resource.name.clone()) {
                        self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AssignTask {
                            task_id: *task_id,
                            resource_name: resource.name,
                        }}).unwrap_or_else(|e| {
                            gui_log!(self, "Failed to assign task to resource: {e}");
                        });
                    }
                }
            }
            if let Some(_watchers_menu) = ui.begin_menu("Watchers") {
                /* list all the resources as a menu item. If the resource is already a watcher, it should be checked */
                let mut resources: Vec<_> = self.project.flow_state().resources.values().cloned().collect();
                resources.sort_by(|a, b| a.name.cmp(&b.name));
                for resource in resources {
                    let is_watching = resource.watched_tasks.contains(task_id);
                    if ui.menu_item_config(resource.name.clone()).selected(is_watching).build() {
                        if is_watching {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::RemoveWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}).unwrap_or_else(|e| {
                                gui_log!(self, "Failed to unwatch task: {e}");
                            });
                        } else {
                            self.project.invoke_command(Command { timestamp: self.get_timestamp(), details: CommandDetails::AddWatcher {
                                task_id: *task_id,
                                resource_name: resource.name.clone(),
                            }}).unwrap_or_else(|e| {
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
                }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
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
                                }}).unwrap_or_else(|e| {
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
                                }}).unwrap_or_else(|e| {
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
                                }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                }
            }
        }
    }

    fn draw_gantt_chart_resources_team_unassigned_task_content_popup(&mut self, ui: &Ui, task_id: &TaskId, task: &Task, day: &NaiveDate) {
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
                        }}).unwrap_or_else(|e| {
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
                        }}).unwrap_or_else(|e| {
                            eprintln!("Failed to update task: {e}");
                        });
                    }
                }
            }
        }
    }

    fn draw_gantt_chart_tasks_task(&mut self, ui: &Ui, task_id: &TaskId, task: &Task) {
        ui.table_next_row();
        ui.table_next_column();

        let _task_token_id = ui.push_id_int(*task_id as i32);
        let task_repr = format!("{} - {}", task.ticket, task.title);
        let task_repr_cstr = std::ffi::CString::new(task_repr.clone()).unwrap();
        let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_DefaultOpen;
        let expand_task = unsafe {
            let bold = self.bold_font.borrow().unwrap();
            let _h = ui.push_font(bold);
            let bg_color = if self.drawing_aids.row_counter % 2 == 0 {
                ui.style_color(StyleColor::TableRowBg)
            } else {
                ui.style_color(StyleColor::TableRowBgAlt)
            };
            ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
            imgui::sys::igTreeNodeEx_Str(task_repr_cstr.as_ptr(), flags as i32)
        };
        //self.draw_gantt_chart_tasks_task_popup(ui, task_id, task);

        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let day = self.project.flow_state().cache().day(i - 1);
                self.draw_cell_background(ui, &day);
                self.draw_milestone(ui, &day);
            }
        }

        if expand_task {
            let mut resource_data: BTreeMap<ResourceId, RoleOfResourceInTask> = self.project.flow_state().worklogs.get(task_id)
                .map(|worklogs| {
                    worklogs.keys()
                        .filter_map(|&resource_id| {
                            self.project.flow_state().resources.get(&resource_id)
                                .map(|resource| (resource_id, RoleOfResourceInTask::WorklogContributor))
                        })
                        .collect()
                })
                .unwrap_or_else(BTreeMap::new);
            if let Some(assignee) = task.assignee {
                resource_data.insert(assignee, RoleOfResourceInTask::Assignee);
            }
            for watcher in &task.watchers {
                resource_data.insert(*watcher, RoleOfResourceInTask::Watcher);
            }

            let resource_entries: BTreeMap<ResourceId, (Resource, RoleOfResourceInTask)> = resource_data
                .into_iter()
                .filter_map(|(resource_id, role)| {
                    self.project.flow_state().resources.get(&resource_id)
                        .map(|resource| (resource_id, (resource.clone(), role)))
                })
                .collect();
            for (resource_id, (resource, role)) in resource_entries {
                self.draw_gantt_chart_tasks_task_resource(ui, task_id, task, &resource_id, &resource, role);
            }
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_gantt_chart_tasks_task_resource(&mut self, ui: &Ui, task_id: &TaskId, task: &Task, resource_id: &ResourceId, resource: &Resource, role: RoleOfResourceInTask) {
        ui.table_next_row();
        ui.table_next_column();
        
        let resource_name_cstr = std::ffi::CString::new(resource.name.clone()).unwrap();
        let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_DefaultOpen;
        let expand_resource = unsafe {
            let bg_color = if self.drawing_aids.row_counter % 2 == 0 {
                ui.style_color(StyleColor::TableRowBg)
            } else {
                ui.style_color(StyleColor::TableRowBgAlt)
            };
            ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
            match role {
                RoleOfResourceInTask::Assignee => {
                    let resource_name_cstr = std::ffi::CString::new(resource.name.clone()).unwrap();
                    imgui::sys::igTreeNodeEx_Str(resource_name_cstr.as_ptr(), (flags | imgui::sys::ImGuiTreeNodeFlags_Bullet) as i32)
                },
                RoleOfResourceInTask::WorklogContributor => {
                    imgui::sys::igTreeNodeEx_Str(resource_name_cstr.as_ptr(), (flags | imgui::sys::ImGuiTreeNodeFlags_Leaf) as i32)
                },
                RoleOfResourceInTask::Watcher => {
                    let disabled_color = ui.style_color(StyleColor::TextDisabled);
                    let _style = ui.push_style_color(imgui::StyleColor::Text, disabled_color);
                    imgui::sys::igTreeNodeEx_Str(resource_name_cstr.as_ptr(), (flags | imgui::sys::ImGuiTreeNodeFlags_Leaf) as i32)
                },
            }
        };
        self.draw_gantt_chart_resources_team_resource_popup(ui, resource_id, &resource);

        for i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let _id = ui.push_id_usize(i);
                let day = self.project.flow_state().cache().day(i - 1);
                self.draw_cell_background(ui, &day);
                self.draw_absence(ui, &day, resource_id, &resource);
                if !self.gui_config.hide_worklogs {
                    self.draw_worklog(ui, &day, resource_id, &resource, task_id, task);
                }
                self.draw_milestone(ui, &day);
                ui.invisible_button("##invisible_button", [-1.0, unsafe { igGetTextLineHeight() }]);
                self.draw_gantt_chart_resources_team_resource_content_popup(ui, resource_id, &resource, &day);
            }
        }
        if expand_resource {
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn apply_pending_draws(&mut self, ui: &Ui) {
        let draw_list = ui.get_window_draw_list();
        for (pos, color, text) in self.drawing_aids.pending_draws.drain(..) {
            draw_list.with_clip_rect_intersect(
                [f32::NEG_INFINITY, f32::NEG_INFINITY],
                [f32::INFINITY, f32::INFINITY],
                || {
                    draw_list.add_text(pos, color, &text);
                },
            );
        }
        self.drawing_aids.pending_draws.clear();
    }

    fn open_task_in_jira(&mut self, ui:& Ui, task: &Task) {
        let jira_url = format!("https://jiradc.ext.net.nokia.com/browse/{}", task.ticket);
        webbrowser::open(&jira_url).unwrap_or_else(|e| {
            gui_log!(self, "Failed to open JIRA URL: {}", e);
        });
    }

    fn get_timestamp(&self) -> DateTime<Utc> {
        if self.date_offset < 0 {
            Utc::now() - chrono::Duration::days(self.date_offset.abs() as i64)
        } else {
            Utc::now() + chrono::Duration::days(self.date_offset.abs() as i64)
        }
    }
}

enum RoleOfResourceInTask {
    Assignee,
    WorklogContributor,
    Watcher,
}

#[derive(Debug, Serialize, Deserialize)]
struct GuiConfig {
    pub config_filename: String,
    pub ticket_prefix: String,
    pub hide_worklogs: bool,
    pub debug_mode: bool,
    pub recent_project_files: Vec<String>,
}

impl GuiConfig {
    fn load_from_yaml(path: &str) -> Self {
        if let Ok(contents) = std::fs::read_to_string(path) {
            if let Ok(config) = serde_yaml::from_str::<GuiConfig>(&contents) {
                return config;
            }
        }
        GuiConfig {
            config_filename: path.to_string(),
            ticket_prefix: "PROJ-".to_string(),
            hide_worklogs: false,
            debug_mode: false,
            recent_project_files: Vec::new(),
        }
    }

    fn save_to_file(&self) {
        if let Ok(contents) = serde_yaml::to_string(self) {
            let _ = std::fs::write(&self.config_filename, contents);
        }
    }
}