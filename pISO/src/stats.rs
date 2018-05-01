use action;
use bitmap;
use controller;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error;
use font;
use input;
use lvm;
use render;

pub struct Stats {
    pub vg: lvm::VolumeGroup,
    pub window: WindowId,
}

impl Stats {
    pub fn new(disp: &mut DisplayManager, vg: lvm::VolumeGroup) -> error::Result<Stats> {
        let window = disp.add_child(Position::Fixed(119, 0))?;
        Ok(Stats {
            window: window,
            vg: vg,
        })
    }
}

impl render::Render for Stats {
    fn render(&self, _window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(0, 0);
        let percent_free = 100.0 - self.vg.pool()?.data_percent;
        let contents = font::render_text(format!("{}% Free", percent_free));
        base.blit(&contents, (0, 0));
        Ok(base.rotate(bitmap::Direction::Left))
    }
}

impl input::Input for Stats {
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

impl Widget for Stats {
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
