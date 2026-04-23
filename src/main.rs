mod gui;
mod app_next;
mod support;

fn main() {
    let gui = gui::Gui::new();
    gui.run();
}
