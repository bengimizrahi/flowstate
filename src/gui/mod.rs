use crate::app::*;
use crate::support;

mod constants;
use constants::*;
mod menu_bar;
mod ribbon;
mod tab_bar;
mod gantt_chart;
mod gantt_chart_resources;
mod gantt_chart_tasks;

#[macro_export]
macro_rules! gui_log {
    ($gui:expr, $($arg:tt)*) => {
        $gui.log(format!($($arg)*))
    };
}

use imgui::*;
use imgui::sys::*;
use chrono::{Utc, DateTime, Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub struct Gui {
    gui_config: GuiConfig,
    project: Project,

    filtered_labels: Vec<LabelId>,
    selected_filter: Option<FilterId>,
    inspections: Vec<TaskInspection>,

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

impl Gui {
    pub fn new() -> Self {
        let gui_config = GuiConfig::load_from_yaml("config.yaml");
        let yaml_filename = gui_config.recent_project_files.first().cloned().unwrap_or_else(|| "database.yaml".to_string());
        Gui {
            gui_config,
            project: Project::load_from_yaml(&yaml_filename,  Utc::now().date_naive()).unwrap_or_else(|e| {
                eprintln!("Failed to load project: {e}");
                Project::new(&yaml_filename)
            }),

            filtered_labels: Vec::new(),
            selected_filter: None,
            inspections: Vec::new(),

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
            "FlowState v1.1.0",
            move |ctx, renderer, _| {
                ctx.set_ini_filename(Some(std::path::PathBuf::from("imgui.ini")));
                let mut bold_font_handle = bold_font_for_init.borrow_mut();
                *bold_font_handle = Some(ctx.fonts().add_font(&[FontSource::TtfData {
                    data: include_bytes!("../../res/Roboto-Bold.ttf"),
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

    fn draw_debug(&mut self, ui: &Ui) {
        let flow_state_str = format!("{:#?}", self.project.flow_state());
        ui.text(flow_state_str);
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

    fn draw_absence(&mut self, ui: &Ui, day: &NaiveDate, resource_id: &ResourceId, _resource: &Resource) {
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

    fn draw_alloc(&mut self, ui: &Ui, worklog: Option<Worklog>, alloc: Option<u8>) {
        let cell_height = unsafe { igGetTextLineHeight() };
        let cell_padding = unsafe { ui.style().cell_padding };
        let effective_cell_height = cell_height + (cell_padding[1]);
        let effective_cell_width = ui.current_column_width();

        if let Some(alloc) = alloc {
            let cursor_pos = unsafe {
                let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                igGetCursorScreenPos(&mut pos);
                pos.y -= cell_padding[1] / 2.0;
                pos
            };

            let alloc_height = effective_cell_height * (alloc as f32 / 100.0);
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

    fn draw_alloc_as_watcher(&mut self, ui: &Ui, day: &NaiveDate, resource_id: Option<&ResourceId>, task_id: &TaskId, _task: &Task) {
        let cell_height = unsafe { igGetTextLineHeight() };
        let cell_padding = unsafe { ui.style().cell_padding };
        let effective_cell_height = cell_height + (cell_padding[1]);
        let effective_cell_width = ui.current_column_width();

        let worklog = self.project.flow_state().worklogs.get(task_id)
            .and_then(|r| r.get(resource_id.unwrap_or(&0)))
            .and_then(|r| r.get(day));
        let alloc = if let Some(_resource_id) = resource_id {
            self.project.flow_state().cache().task_alloc_rendering.get(task_id)
                .and_then(|r| r.get(day))
        } else {
            self.project.flow_state().cache().task_alloc_rendering.get(task_id)
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

    fn draw_worklog(&mut self, ui: &Ui, day: &NaiveDate, resource_id: &ResourceId, _resource: &Resource, task_id: &TaskId, _task: &Task) {
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

    fn draw_worklog_on_others_tasks(&mut self, ui: &Ui, day: &NaiveDate, resource_id: &ResourceId, _resource: &Resource) {
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
        let _today = chrono::Local::now().date_naive();
        if let Some(_milestones) = self.project.flow_state().cache().date_to_milestones.get(day) {
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
            
            let transparent_pink = [1.0, 0.0, 0.0, 0.0];
            let opaque_pink = [1.0, 0.0, 0.0, 0.7];
            
            // Draw gradient rectangle for milestone indicator
            draw_list.add_rect_filled_multicolor(
                gradient_start,
                gradient_end,
                transparent_pink,
                opaque_pink,
                opaque_pink,
                transparent_pink,
            );
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

    fn open_task_in_jira(&mut self, _ui:& Ui, task: &Task) {
        let jira_url = format!("https://jiradc.ext.net.nokia.com/browse/{}", task.ticket);
        webbrowser::open(&jira_url).unwrap_or_else(|e| {
            gui_log!(self, "Failed to open JIRA URL: {}", e);
        });
    }

    fn get_timestamp(&self) -> DateTime<Utc> {
        Utc::now() + chrono::Duration::days(self.date_offset as i64)
    }

    fn draw_inspection_tab(&mut self, ui: &Ui, inspection: &TaskInspection) {
        let task = inspection.flow_state.tasks.get(&inspection.task_id).unwrap();
        let task_label = format!("{} - {}", 
            task.ticket,
            task.title.chars().take(20).collect::<String>()
        );

        let mut open = true;
        if let Some(_tab_token) = TabItem::new(&task_label)
            .opened(&mut open)
            .begin(ui)
        {
            let _id = ui.push_id_usize(inspection.task_id as usize);
            if self.draw_inspection_table(ui, inspection, "##inspection_gantt_chart") {
                self.draw_inspection_calendar_row(ui, inspection);
                self.draw_inspection_milestones_row(ui, inspection);
                self.draw_inspection_content(ui, inspection);
                unsafe { imgui::sys::igEndTable(); }
            }
            // `_tab_token` drops here (EndTabItem)
        }

        if !open {
            self.inspections.retain(|insp| insp.task_id != inspection.task_id);
        }
    }

    fn draw_inspection_table(&mut self, _ui: &Ui, inspection: &TaskInspection, id: &str) -> bool {
        let table_id = std::ffi::CString::new(id).unwrap();
        let flags = imgui::sys::ImGuiTableFlags_Borders
            | imgui::sys::ImGuiTableFlags_HighlightHoveredColumn
            | imgui::sys::ImGuiTableFlags_SizingFixedFit
            | imgui::sys::ImGuiTableFlags_ScrollX
            | imgui::sys::ImGuiTableFlags_ScrollY
            | imgui::sys::ImGuiTableFlags_Resizable
            | imgui::sys::ImGuiTableFlags_NoPadOuterX
            | imgui::sys::ImGuiTableFlags_NoPadInnerX;
        let num_columns = inspection.flow_state.cache().num_days() + 1;
        unsafe {imgui::sys::igBeginTable(
            table_id.as_ptr(),
            num_columns as i32,
            flags as i32,
            imgui::sys::ImVec2 { x: 0.0, y: 0.0 },
            0.0,
        )}
    }

    fn draw_inspection_calendar_row(&mut self, ui: &Ui, inspection: &TaskInspection) {
        let mut table_column_data = TableColumnSetup::new("Calendar");
        table_column_data.flags = TableColumnFlags::NO_HIDE | TableColumnFlags::NO_REORDER;
        ui.table_setup_column_with(table_column_data);
        for i in 0..inspection.flow_state.cache().num_days() {
            let day: chrono::NaiveDate = inspection.flow_state.cache().day(i);
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
       
        let today = self.get_timestamp().date_naive();
        for i in 0..inspection.flow_state.cache().num_days() {
            let day: chrono::NaiveDate = inspection.flow_state.cache().day(i);
            if day == today {
                let pink = [1.0, 0.75, 0.8, 1.0];
                ui.table_set_bg_color_with_column(TableBgTarget::CELL_BG, pink, i + 1);
            }
        }
    }

    fn draw_inspection_milestones_row(&mut self, ui: &Ui, inspection: &TaskInspection) {
        ui.table_next_row();
        ui.table_next_column();
        ui.text("  Milestones");
        for i in 1..=inspection.flow_state.cache().num_days() {
            if ui.table_next_column() {
                let _id = ui.push_id_usize(i);
                let day = inspection.flow_state.cache().day(i - 1);
                if let Some(milestones) = inspection.flow_state.cache().date_to_milestones.get(&day) {
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
        for _i in 1..=self.project.flow_state().cache().num_days() {
            if ui.table_next_column() {
                let bg_color = ui.style_color(StyleColor::TableHeaderBg);
                ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
            }
        }
    }

    fn draw_inspection_content(&mut self, ui: &Ui, inspection: &TaskInspection) {
        self.drawing_aids.previous_rect = None;
        self.drawing_aids.previous_assignee_in_inspection = None;
        for i in 0..inspection.flow_state.cache().num_days() {
            let day = inspection.flow_state.cache().day(i);
            if day < inspection.start_date {
                continue;
            }
            if self.gui_config.hide_weekends_in_inspection {
                if day.weekday() == chrono::Weekday::Sat || day.weekday() == chrono::Weekday::Sun {
                    continue;
                }
            }
            if day > self.get_timestamp().date_naive() {
                break;
            }
            self.draw_inspection_content_for_day(ui, inspection, &day);
        }
    }

    fn draw_inspection_content_for_day(&mut self, ui: &Ui, inspection: &TaskInspection, date: &NaiveDate) {
        let _vday_token_id = ui.push_id(date.to_string());
        let assignee = inspection.assignee_history.get(date).copied().flatten();
    
        let worklogs = inspection.worklogs_history.get(date);
        let allocs = inspection.allocations_history.get(date);
        let absences = inspection.absences_history.get(date);

        if self.gui_config.hide_non_deviations_in_inspection {
            if let Some(assignee) = assignee {
                let worklog = inspection.flow_state.worklogs.get(&inspection.task_id)
                        .and_then(|wl_map| wl_map.get(&assignee))
                        .and_then(|wl_date_map| wl_date_map.get(date));
                let absence = inspection.flow_state.cache().resource_absence_rendering.get(&assignee)
                        .and_then(|abs_map| abs_map.get(date))
                        .copied();
                let w = match worklog {
                    Some(wl) => wl.fraction,
                    None => 0,
                };
                let a = match absence {
                    Some(abs) => abs,
                    None => 0,
                };
                if (w == 100 && a == 0) || (w == 0 && a == 100) {
                    return;
                }
            }
        }

        ui.table_next_row();
        ui.table_next_column();

        let assignee_name = assignee
            .and_then(|id| self.project.flow_state().resources.get(&id))
            .map(|r| r.name.clone())
            .unwrap_or_else(|| "Unassigned".to_string());
        let assignee_cstr = std::ffi::CString::new(assignee_name).unwrap();
        let expand_task = unsafe {
            if assignee == self.drawing_aids.previous_assignee_in_inspection {
                let empty_cstr = std::ffi::CString::new(String::from("")).unwrap();
                let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_Leaf;
                imgui::sys::igTreeNodeEx_Str(empty_cstr.as_ptr(), flags as i32)
            } else {    
                let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_Bullet;
                imgui::sys::igTreeNodeEx_Str(assignee_cstr.as_ptr(), flags as i32)
            }
        };
        self.drawing_aids.previous_assignee_in_inspection = assignee;

        for i in 1..=inspection.flow_state.cache().num_days() {
            let day = inspection.flow_state.cache().day(i - 1);
            if ui.table_next_column() {
                let _hday_token_id = ui.push_id_usize(i);
                self.draw_cell_background(ui, &day);
                if !self.gui_config.hide_worklogs {   
                    if let Some(worklog) = worklogs.and_then(|wl_map| wl_map.get(&day)).copied() {
                        self.draw_inspection_worklog(ui, worklog);
                    }
                }
                if let Some(absence) = absences.and_then(|abs_map| abs_map.get(&day)).copied() {
                    self.draw_inspection_absence(ui, absence);
                }
                let alloc = allocs.and_then(|alloc_map| alloc_map.get(&day)).copied();
                let worklog = worklogs.and_then(|wl_map| wl_map.get(&day)).copied();
                self.draw_inspection_alloc(ui, worklog, alloc);
            }
            self.draw_milestone(ui, &day);
            if date == &day {
                self.draw_inspection_current_day(ui, &day);
            }
        }
        if expand_task {
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_inspection_worklog(&mut self, ui: &Ui, worklog: u8) {
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
        let worklog_height = effective_cell_height * (worklog as f32) / 100.0;
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

    fn draw_inspection_absence(&mut self, ui: &Ui, absence: u8) {
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

        let absence_height = (effective_cell_height * (absence as f32 / 100.0)).max(1.0);
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

    fn draw_inspection_alloc(&mut self, ui: &Ui, worklog: Option<u8>, alloc: Option<u8>) {
        let cell_height = unsafe { igGetTextLineHeight() };
        let cell_padding = unsafe { ui.style().cell_padding };
        let effective_cell_height = cell_height + (cell_padding[1]);
        let effective_cell_width = ui.current_column_width();

        if let Some(alloc) = alloc {
            let cursor_pos = unsafe {
                let mut pos = ImVec2 { x: 0.0, y: 0.0 };
                igGetCursorScreenPos(&mut pos);
                pos.y -= cell_padding[1] / 2.0;
                pos
            };

            let alloc_height = effective_cell_height * (alloc as f32 / 100.0);
            let worklog_height = if let Some(worklog) = worklog {
                effective_cell_height * (worklog as f32) / 100.0
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

    fn draw_inspection_current_day(&mut self, ui: &Ui, _day: &NaiveDate) {
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
        let gradient_start = [cursor_pos.x + (0.1 * effective_cell_width), cursor_pos.y];
        let gradient_end = [cursor_pos.x + effective_cell_width, cursor_pos.y + effective_cell_height];
        
        let transparent_red = [1.0, 0.75, 0.8, 0.0];
        let opaque_red = [1.0, 0.75, 0.8, 1.0];
        
        // Draw gradient rectangle for milestone indicator
        draw_list.add_rect_filled_multicolor(
            gradient_start,
            gradient_end,
            opaque_red,
            transparent_red,
            transparent_red,
            opaque_red,
        );
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
    pub hide_weekends_in_inspection: bool,
    pub hide_non_deviations_in_inspection: bool,
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
            hide_weekends_in_inspection: false,
            hide_non_deviations_in_inspection: false,
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
struct DrawingAids {
    previous_rect: Option<(ImVec2, ImVec2)>,
    row_counter: usize,
    pending_draws: Vec<([f32; 2], [f32; 4], String)>,
    previous_assignee_in_inspection: Option<ResourceId>,
}

impl DrawingAids {
    pub fn new() -> Self {
        DrawingAids { previous_rect: None, row_counter: 0, pending_draws: Vec::new(), previous_assignee_in_inspection: None }
    }
}