use controller;

pub trait Input {
    fn on_event(&mut self, event: &controller::Event) -> bool {
        true
    }
}
