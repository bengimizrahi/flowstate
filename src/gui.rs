use crate::app::*;
pub struct Gui {
    application: Application,
}

impl Gui {
    pub fn new() -> Self {
        Gui {
            application: Application::new(),
        }
    }

    pub fn run(&self) {
        // Placeholder for GUI run logic
        println!("Running GUI...");
    }
}
