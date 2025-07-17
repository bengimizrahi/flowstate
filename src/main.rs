mod flowstate;
mod gui;
mod support;

fn main() {
    let gui = gui::Gui::new();
    gui.run();
}
