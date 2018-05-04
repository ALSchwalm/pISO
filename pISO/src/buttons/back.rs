use action;
use bitmap;
use controller;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error;
use font;
use input;
use render;

pub struct BackButton {
    action: action::Action,
    pub windowid: WindowId,
}

impl BackButton {
    pub fn new(disp: &mut DisplayManager, action: action::Action) -> error::Result<BackButton> {
        let our_window = disp.add_child(Position::Normal)?;
        Ok(BackButton {
            windowid: our_window,
            action: action,
        })
    }
}

impl render::Render for BackButton {
    fn render(&self, _manager: &DisplayManager, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(&font::render_text("Back"), (12, 0));
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for BackButton {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => Ok((true, vec![self.action.clone()])),
            _ => Ok((false, vec![])),
        }
    }
}

impl Widget for BackButton {
    fn windowid(&self) -> WindowId {
        self.windowid
    }
}
