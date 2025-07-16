use imgui::*;
use crate::support;
use crate::flowstate;

pub struct Gui {
    flow_state: flowstate::FlowState,
}

impl Gui {
    pub fn new() -> Self {
        Gui { flow_state: flowstate::FlowState::new() }
    }

    pub fn run(mut self) {
        support::simple_init(file!(), move |_, ui| {
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
        self.draw_top_ribbon(ui);
        self.draw_gantt_chart(ui);
    }

    fn draw_menu_bar(&mut self, ui: &Ui) {
        ui.menu_bar(|| {
            ui.menu("File", || {
                if ui.menu_item("New Project...") {

                }
                if ui.menu_item("Open Project...") {

                }
                if ui.menu_item("Save Project") {

                }
                if ui.menu_item("Exit") {

                }
            });
            ui.menu("Edit", || {
                if ui.menu_item("Undo") {

                }
                if ui.menu_item("Redo") {

                }
            });
            ui.menu("Action", || {
                if ui.menu_item("Team") {

                }
            });
            ui.menu("Filter", || {
            });
            ui.menu("Help", || {
                if ui.menu_item("About") {

                }
            });
        });
    }

    fn draw_top_ribbon(&mut self, ui: &Ui) {

    }

    fn draw_gantt_chart(&mut self, ui: &Ui) {
        if let Some(_tab_bar) = ui.tab_bar("##tab_bar") {
            if let Some(_res_tab_item) = ui.tab_item("Resources"){
                self.draw_resources_gantt_chart(ui);
            }
            if let Some(task_tab_item) = ui.tab_item("Tasks") {
                self.draw_tasks_tab_gantt_chart(ui);
            }
        }
    }

    fn draw_resources_gantt_chart(&mut self, ui: &Ui) {
        self.draw_resources_gantt_chart_header_row(ui);
        self.draw_milestones_row(ui);
        self.draw_resources_gantt_chart_contents(ui);
    }

    fn draw_resources_gantt_chart_header_row(&mut self, ui: &Ui) {
        let table_id = std::ffi::CString::new("##resources_table").unwrap();
        let flags = imgui::sys::ImGuiTableFlags_Borders
            | imgui::sys::ImGuiTableFlags_HighlightHoveredColumn
            | imgui::sys::ImGuiTableFlags_SizingFixedFit
            | imgui::sys::ImGuiTableFlags_ScrollX
            | imgui::sys::ImGuiTableFlags_ScrollY
            | imgui::sys::ImGuiTableFlags_Resizable
            | imgui::sys::ImGuiTableFlags_NoPadOuterX
            | imgui::sys::ImGuiTableFlags_NoPadInnerX;
        if unsafe {imgui::sys::igBeginTable(
            table_id.as_ptr(),
            (self.flow_state.l1().num_days() + 1) as i32,
            flags as i32,
            imgui::sys::ImVec2 { x: 0.0, y: 0.0 },
            0.0,
        )} {
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
                if (day == today) {
                    let pink = [1.0, 0.75, 0.8, 1.0];
                    ui.table_set_bg_color_with_column(TableBgTarget::CELL_BG, pink, i + 1);
                }
            }
        }

        unsafe {imgui::sys::igEndTable();}
    }

    fn draw_resources_gantt_chart_contents(&mut self, ui: &Ui) {

    }
    fn draw_tasks_tab_gantt_chart(&mut self, ui: &Ui) {

    }

    fn draw_milestones_row(&mut self, ui: &Ui) {
        ui.table_next_row();
        ui.table_next_column();
        ui.text("  Milestones");
        for i in 0..self.flow_state.l1().num_days() {
        }
    }
}
