use action;
use controller;
use displaymanager::DisplayManager;
use error;

pub trait Input {
    fn on_event(
        &mut self,
        _event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        Ok((false, vec![]))
    }

    fn do_action(
        &mut self,
        _disp: &mut DisplayManager,
        _action: &action::Action,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        Ok((false, vec![]))
    }
}
