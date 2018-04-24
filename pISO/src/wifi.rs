use action;
use bitmap;
use config;
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
    config: config::Config,
    pub windowid: WindowId,
}

impl WifiMenu {
    pub fn new(disp: &mut DisplayManager, config: &config::Config) -> error::Result<WifiMenu> {
        Ok(WifiMenu {
            windowid: disp.add_child(Position::Normal)?,
            state: WifiMenuState::Closed,
            config: config.clone(),
        })
    }
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
                let menu = SelectWifiMenu::new(disp, &self.config.clone())?;
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
    pub windowid: WindowId,
    clients: Vec<WifiClient>,
    // ap: WifiAp,
    config: config::Config,
}

impl SelectWifiMenu {
    fn new(disp: &mut DisplayManager, config: &config::Config) -> error::Result<SelectWifiMenu> {
        let window = disp.add_child(Position::Fixed(0, 0))?;
        let clients = config
            .wifi
            .client
            .iter()
            .map(|config| {
                WifiClient::new(disp, config.clone())
                    .expect("Failed to create WifiClient menu item")
            })
            .collect::<Vec<_>>();

        //TODO: there may not be any clients
        disp.shift_focus(clients.first().unwrap());
        Ok(SelectWifiMenu {
            windowid: window,
            config: config.clone(),
            clients: clients,
        })
    }
}

impl<'a> render::Render for SelectWifiMenu {
    fn render(&self, window: &Window) -> error::Result<bitmap::Bitmap> {
        Ok(bitmap::Bitmap::new(0, 0))
    }
}

impl<'a> input::Input for SelectWifiMenu {}

impl<'a> Widget for SelectWifiMenu {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        self.clients
            .iter_mut()
            .map(|item| item as &mut Widget)
            .collect()
    }

    fn children(&self) -> Vec<&Widget> {
        self.clients.iter().map(|item| item as &Widget).collect()
    }

    fn windowid(&self) -> WindowId {
        self.windowid
    }
}

pub struct WifiClient {
    pub windowid: WindowId,
    config: config::WifiClientNetworkConfig,
}

impl WifiClient {
    fn new(
        disp: &mut DisplayManager,
        config: config::WifiClientNetworkConfig,
    ) -> error::Result<WifiClient> {
        Ok(WifiClient {
            windowid: disp.add_child(Position::Normal)?,
            config: config,
        })
    }
}

impl render::Render for WifiClient {
    fn render(&self, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(&font::render_text(&self.config.ssid), (12, 0));
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for WifiClient {}

impl Widget for WifiClient {
    fn windowid(&self) -> WindowId {
        self.windowid
    }
}

pub struct WifiAp {
    pub windowid: WindowId,
    config: config::WifiApConfig,
}
