use action;
use bitmap;
use controller;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error;
use font;
use input;
use render;
use std::sync::{Arc, Mutex};

pub struct NewDrive {
    pub window: WindowId,
}

impl NewDrive {
    pub fn new(disp: &mut DisplayManager, parent: WindowId) -> error::Result<NewDrive> {
        let our_window = disp.add_child(parent, Position::Normal)?;
        Ok(NewDrive { window: our_window })
    }
}

impl render::Render for NewDrive {
    fn render(&self, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(font::render_text("New Drive"), (10, 0));
        if window.focus {
            base.blit(bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for NewDrive {
    fn on_event(&mut self, event: &controller::Event) -> (bool, Vec<action::Action>) {
        match *event {
            controller::Event::Select => {
                (true, vec![action::Action::CreateDrive(12 * 1024 * 1024)])
            }
            _ => (false, vec![]),
        }
    }

    fn do_action(
        &mut self,
        disp: &mut DisplayManager,
        action: &action::Action,
    ) -> error::Result<bool> {
        Ok(false)
    }
}

impl Widget for NewDrive {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        vec![]
    }

    fn children(&self) -> Vec<&Widget> {
        vec![]
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}
