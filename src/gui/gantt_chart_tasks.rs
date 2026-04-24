use crate::gui::*;

enum RoleOfResourceInTask {
    Assignee,
    WorklogContributor,
    Watcher,
}

impl Gui {
    pub(super) fn draw_gantt_chart_tasks(&mut self, ui: &Ui) {
        if self.draw_gantt_chart_table(ui, "##tasks_gantt_chart") {
            self.draw_gantt_chart_calendar_row(ui);
            self.draw_gantt_chart_milestones_row(ui);
            self.draw_gantt_chart_tasks_contents(ui);
            unsafe {imgui::sys::igEndTable();}
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
                                .map(|_resource| (resource_id, RoleOfResourceInTask::WorklogContributor))
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
}