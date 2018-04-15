use action;
use bitmap;
use controller;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error;
use font;
use input;
use render;

enum WifiMenuState {
    Closed,
    Open(SelectWifiMenu),
}

pub struct WifiMenu {
    state: WifiMenuState,
    pub windowid: WindowId,
}

impl render::Render for WifiMenu {
    fn render(&self, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(&font::render_text("WiFi"), (12, 0));
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for WifiMenu {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => Ok((true, vec![action::Action::OpenWifiMenu])),
            _ => Ok((false, vec![])),
        }
    }

    fn do_action(
        &mut self,
        disp: &mut DisplayManager,
        action: &action::Action,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *action {
            action::Action::OpenWifiMenu => {
                let menu = SelectWifiMenu::new()?;
                disp.shift_focus(&menu);
                self.state = WifiMenuState::Open(menu);
                Ok((true, vec![]))
            }
            _ => Ok((false, vec![])),
        }
    }
}

impl Widget for WifiMenu {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        match self.state {
            WifiMenuState::Open(ref mut widget) => vec![widget],
            _ => vec![],
        }
    }

    fn children(&self) -> Vec<&Widget> {
        match self.state {
            WifiMenuState::Open(ref widget) => vec![widget],
            _ => vec![],
        }
    }

    fn windowid(&self) -> WindowId {
        self.windowid
    }
}

pub struct SelectWifiMenu {
    pub window: WindowId,
    clients: Vec<WifiClient>,
    ap: WifiAp,
}

pub struct WifiClient {
    pub window: WindowId,
}

pub struct WifiAp {
    pub window: WindowId,
}
