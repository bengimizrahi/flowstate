use crate::gui::*;
use crate::gui::utils::*;
use crate::gui_log;

pub struct Gui {
    pub(super) gui_config: GuiConfig,
    pub(super) project: Project,

    pub(super) filtered_labels: Vec<LabelId>,
    pub(super) selected_filter: Option<FilterId>,
    pub(super) inspections: Vec<TaskInspection>,

    pub(super) date_offset: i32,

    pub(super) bold_font: std::rc::Rc<std::cell::RefCell<Option<FontId>>>,
    pub(super) find_input_buffer: String,
    pub(super) new_project_input_text_buffer: String,
    pub(super) team_input_text_buffer: String,
    pub(super) resource_input_text_buffer: String,
    pub(super) ticket_input_text_buffer: String,
    pub(super) task_title_input_text_buffer: String,
    pub(super) task_duration_days: f32,
    pub(super) absence_duration_days: f32,
    pub(super) worklog_fraction: u8,
    pub(super) milestone_input_text_buffer: String,
    pub(super) milestone_date_input_text_buffer: String,
    pub(super) label_input_text_buffer: String,
    pub(super) filter_input_text_buffer: String,
    pub(super) logs: Vec<String>,
    pub(super) drawing_aids: DrawingAids,
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

    pub(super) fn log(&mut self, message: String) {
        self.logs.push(message);
        if self.logs.len() > 10 {
            self.logs.drain(0..self.logs.len() - 10);
        }
    }

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

    pub(super) fn draw_debug(&mut self, ui: &Ui) {
        let flow_state_str = format!("{:#?}", self.project.flow_state());
        ui.text(flow_state_str);
    }

    pub(super) fn open_task_in_jira(&mut self, _ui:& Ui, task: &Task) {
        let jira_url = format!("https://jiradc.ext.net.nokia.com/browse/{}", task.ticket);
        webbrowser::open(&jira_url).unwrap_or_else(|e| {
            gui_log!(self, "Failed to open JIRA URL: {}", e);
        });
    }

    pub(super) fn get_timestamp(&self) -> DateTime<Utc> {
        Utc::now() + chrono::Duration::days(self.date_offset as i64)
    }
}