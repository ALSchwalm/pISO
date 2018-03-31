use action;
use controller;
use error;

pub trait Input {
    fn on_event(&mut self, event: &controller::Event) -> (bool, Vec<action::Action>) {
        (true, vec![])
    }

    fn do_action(&mut self, action: &action::Action) -> error::Result<bool> {
        Ok(false)
    }
}
