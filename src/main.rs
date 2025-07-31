mod gui;
mod app;
mod support;

fn main() {
    let gui = gui::Gui::new();
    gui.run();
}
