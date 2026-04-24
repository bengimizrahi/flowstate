use crate::gui::*;

impl Gui {
    pub(super) fn draw_cell_background(&mut self, ui: &Ui, day: &NaiveDate) {
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

    pub(super) fn draw_absence(&mut self, ui: &Ui, day: &NaiveDate, resource_id: &ResourceId, _resource: &Resource) {
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

    pub(super) fn draw_alloc(&mut self, ui: &Ui, worklog: Option<Worklog>, alloc: Option<u8>) {
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

    pub(super) fn draw_alloc_as_watcher(&mut self, ui: &Ui, day: &NaiveDate, resource_id: Option<&ResourceId>, task_id: &TaskId, _task: &Task) {
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

    pub(super) fn draw_worklog(&mut self, ui: &Ui, day: &NaiveDate, resource_id: &ResourceId, _resource: &Resource, task_id: &TaskId, _task: &Task) {
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

    pub(super) fn draw_worklog_on_others_tasks(&mut self, ui: &Ui, day: &NaiveDate, resource_id: &ResourceId, _resource: &Resource) {
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

    pub(super) fn draw_milestone(&mut self, ui: &Ui, day: &NaiveDate) {
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

    pub(super) fn apply_pending_draws(&mut self, ui: &Ui) {
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
}

pub(super) struct DrawingAids {
    pub(super) previous_rect: Option<(ImVec2, ImVec2)>,
    pub(super) row_counter: usize,
    pub(super) pending_draws: Vec<([f32; 2], [f32; 4], String)>,
    pub(super) previous_assignee_in_inspection: Option<ResourceId>,
}

impl DrawingAids {
    pub fn new() -> Self {
        DrawingAids { previous_rect: None, row_counter: 0, pending_draws: Vec::new(), previous_assignee_in_inspection: None }
    }
}