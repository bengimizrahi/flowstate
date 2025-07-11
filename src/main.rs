mod support;
mod flowstate;

fn main() {
    let _flow_state = flowstate::FlowState::new();
    
    support::simple_init(file!(), move |_, _ui| {
        // nothing! don't actually do any imgui funtimes
    });
}
