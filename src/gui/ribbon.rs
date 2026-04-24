use crate::gui::*;

impl Gui {
    pub(super)fn draw_ribbon(&mut self, ui: &Ui) {
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
            if ui.input_int("##date_offset_input", &mut self.date_offset)
                .step(1)
                .build()
            {
                let timestamp = self.get_timestamp().date_naive();
                self.project.flow_state_mut().rebuild_cache(timestamp);
            }
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
}