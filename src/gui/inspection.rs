use crate::gui::*;

impl Gui {
    pub(super) fn draw_inspection_table(&mut self, _ui: &Ui, inspection: &TaskInspection, id: &str) -> bool {
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

    pub(super) fn draw_inspection_calendar_row(&mut self, ui: &Ui, inspection: &TaskInspection) {
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

    pub(super) fn draw_inspection_milestones_row(&mut self, ui: &Ui, inspection: &TaskInspection) {
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
}