use imgui::*;
use crate::support;
use crate::flowstate;
use chrono::Datelike;

pub struct Gui {
    flow_state: flowstate::state::FlowState,

    create_team_input_text_buffer: String,
}

impl Gui {
    pub fn new() -> Self {
        let mut flow_state = flowstate::state::FlowState::new();
        flow_state.load_from_yaml().unwrap();
        Gui {
            flow_state,
            create_team_input_text_buffer: String::new(),
        }
    }

    pub fn run(mut self) {
        support::simple_init(file!(), move |run, ui| {
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
                if ui.menu_item("Undo") {

                }
                if ui.menu_item("Redo") {

                }
            };
            if let Some(_action_menu) = ui.begin_menu("Insert") {
                if let Some(_team_menu) = ui.begin_menu("Team") {
                    if let Some(_child_window) = ui.child_window("##team_menu")
                            .size([120.0, 20.0])
                            .begin() {
                        let mut can_create_team = false;
                        if ui.input_text("##team_name", &mut self.create_team_input_text_buffer)
                                .enter_returns_true(true)
                                .build() {
                            can_create_team = !self.create_team_input_text_buffer.is_empty();
                        }
                        ui.same_line();
                        if ui.button("Ok") {
                            can_create_team = !self.create_team_input_text_buffer.is_empty();
                        }
                        if can_create_team {
                            self.flow_state.create_team(self.create_team_input_text_buffer.clone()).unwrap();
                            self.create_team_input_text_buffer.clear();
                        } 
                    }
                }
                if ui.menu_item("Resource...") {

                }
                if ui.menu_item("Task...") {

                }
                if ui.menu_item("Milestone...") {

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
                self.draw_resources_gantt_chart(ui);
            }
            if let Some(task_tab_item) = ui.tab_item("Tasks") {
                self.draw_tasks_tab_gantt_chart(ui);
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
        unsafe {imgui::sys::igBeginTable(
            table_id.as_ptr(),
            (self.flow_state.l1().num_days() + 1) as i32,
            flags as i32,
            imgui::sys::ImVec2 { x: 0.0, y: 0.0 },
            0.0,
        )}
    }
    
    fn draw_resources_gantt_chart(&mut self, ui: &Ui) {
        if self.draw_gantt_chart_table(ui, "##resources_gantt_chart") {
            self.draw_gantt_chart_header_row(ui);
            self.draw_milestones_row(ui);
            self.draw_resources_gantt_chart_contents(ui);
            unsafe {imgui::sys::igEndTable();}
        }
    }

    fn draw_tasks_tab_gantt_chart(&mut self, ui: &Ui) {
        if self.draw_gantt_chart_table(ui, "##tasks_gantt_chart") {
            self.draw_gantt_chart_header_row(ui);
            self.draw_milestones_row(ui);
            self.draw_tasks_gantt_chart_contents(ui);
            unsafe {imgui::sys::igEndTable();}
        }
    }

    fn draw_gantt_chart_header_row(&mut self, ui: &Ui) {
        let mut table_data = TableColumnSetup::new("Calendar");
        table_data.flags = TableColumnFlags::NO_HIDE | TableColumnFlags::NO_REORDER;
        ui.table_setup_column_with(table_data);
        for i in 0..self.flow_state.l1().num_days() {
            let day: chrono::NaiveDate = self.flow_state.l1().start_date + chrono::Duration::days(i as i64);
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
        for i in 0..self.flow_state.l1().num_days() {
            let day: chrono::NaiveDate = self.flow_state.l1().start_date + chrono::Duration::days(i as i64);
            if day == today {
                let pink = [1.0, 0.75, 0.8, 1.0];
                ui.table_set_bg_color_with_column(TableBgTarget::CELL_BG, pink, i + 1);
            }
        }
    }

    fn draw_milestones_row(&mut self, ui: &Ui) {
        ui.table_next_row();
        ui.table_next_column();
        ui.text("  Milestones");
        for i in 1..=self.flow_state.l1().num_days() {
            if ui.table_next_column() {
                let _id = ui.push_id_usize(i);
                let day = self.flow_state.l1().day(i - 1);
                if day.weekday() == chrono::Weekday::Sat || day.weekday() == chrono::Weekday::Sun {
                    let bg_color = ui.style_color(StyleColor::TableHeaderBg);
                    ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
                }
            }
        }
    }

    fn draw_resources_gantt_chart_contents(&mut self, ui: &Ui) {
        let team_names: Vec<String> = self.flow_state.teams.iter().cloned().collect();
        for team_name in team_names.iter() {
            self.draw_resource_gantt_chart_team(ui, team_name);
        }
    }

    fn draw_resource_gantt_chart_team(&mut self, ui: &Ui, team_name: &str) {
        ui.table_next_row();
        ui.table_next_column();
        
        let team_name_cstr = std::ffi::CString::new(team_name).unwrap();
        let flags = imgui::sys::ImGuiTreeNodeFlags_SpanFullWidth | imgui::sys::ImGuiTreeNodeFlags_DefaultOpen;
        let expand_team = unsafe {
            imgui::sys::igTreeNodeEx_Str(team_name_cstr.as_ptr(), flags as i32)
        };

        for i in 1..=self.flow_state.l1().num_days() {
            if ui.table_next_column() {
                let _id = ui.push_id_usize(i);
                let day = self.flow_state.l1().day(i - 1);
                if day.weekday() == chrono::Weekday::Sat || day.weekday() == chrono::Weekday::Sun {
                    let bg_color = ui.style_color(StyleColor::TableHeaderBg);
                    ui.table_set_bg_color(TableBgTarget::CELL_BG, bg_color);
                }
            }
        }

        if expand_team {
            unsafe {imgui::sys::igTreePop();}
        }

    }

    fn draw_tasks_gantt_chart_contents(&mut self, ui: &Ui) {
    }

}
