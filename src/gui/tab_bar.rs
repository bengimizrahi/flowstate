use crate::gui::*;

impl Gui {
    pub(super) fn draw_tab_bar(&mut self, ui: &Ui) {
        if let Some(_tab_bar) = ui.tab_bar("##tab_bar") {
            if let Some(_res_tab_item) = ui.tab_item("Resources"){
                self.draw_gantt_chart_resources(ui);
            }
            if let Some(_task_tab_item) = ui.tab_item("Tasks") {
                self.draw_gantt_chart_tasks(ui);
            }
            for inspection in self.inspections.clone() {
                self.draw_inspection_tab(ui, &inspection);
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
}