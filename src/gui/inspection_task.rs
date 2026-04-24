use crate::gui::*;

impl Gui {
    pub(super) fn draw_task_inspection_tab(&mut self, ui: &Ui, inspection: &TaskInspection) {
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
                self.draw_task_inspection_content(ui, inspection);
                unsafe { imgui::sys::igEndTable(); }
            }
            // `_tab_token` drops here (EndTabItem)
        }

        if !open {
            self.inspections.retain(|insp| insp.task_id != inspection.task_id);
        }
    }

    fn draw_task_inspection_content(&mut self, ui: &Ui, inspection: &TaskInspection) {
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
            self.draw_task_inspection_content_for_day(ui, inspection, &day);
        }
    }

    fn draw_task_inspection_content_for_day(&mut self, ui: &Ui, inspection: &TaskInspection, date: &NaiveDate) {
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
                        self.draw_task_inspection_worklog(ui, worklog);
                    }
                }
                if let Some(absence) = absences.and_then(|abs_map| abs_map.get(&day)).copied() {
                    self.draw_task_inspection_absence(ui, absence);
                }
                let alloc = allocs.and_then(|alloc_map| alloc_map.get(&day)).copied();
                let worklog = worklogs.and_then(|wl_map| wl_map.get(&day)).copied();
                self.draw_task_inspection_alloc(ui, worklog, alloc);
            }
            self.draw_milestone(ui, &day);
            if date == &day {
                self.draw_task_inspection_current_day(ui, &day);
            }
        }
        if expand_task {
            unsafe {imgui::sys::igTreePop();}
        }
    }

    fn draw_task_inspection_worklog(&mut self, ui: &Ui, worklog: u8) {
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

    fn draw_task_inspection_absence(&mut self, ui: &Ui, absence: u8) {
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

    fn draw_task_inspection_alloc(&mut self, ui: &Ui, worklog: Option<u8>, alloc: Option<u8>) {
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

    fn draw_task_inspection_current_day(&mut self, ui: &Ui, _day: &NaiveDate) {
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