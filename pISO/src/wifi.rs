use action;
use bitmap;
use buttons;
use config;
use controller;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error;
use font;
use input;
use render;
use utils;

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
    fn render(&self, _manager: &DisplayManager, window: &Window) -> error::Result<bitmap::Bitmap> {
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
            action::Action::CloseWifiMenu => {
                self.state = WifiMenuState::Closed;
                disp.shift_focus(self);
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
    ap: WifiAp,
    back: buttons::back::BackButton,
    _config: config::Config,
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

        let ap = WifiAp::new(disp, config.wifi.ap.clone())?;

        disp.shift_focus(
            clients
                .first()
                .map(|client| client as &Widget)
                .unwrap_or(&ap),
        );
        Ok(SelectWifiMenu {
            windowid: window,
            _config: config.clone(),
            back: buttons::back::BackButton::new(disp, action::Action::CloseWifiMenu)?,
            clients: clients,
            ap: ap,
        })
    }
}

impl<'a> render::Render for SelectWifiMenu {
    fn render(&self, manager: &DisplayManager, _window: &Window) -> error::Result<bitmap::Bitmap> {
        Ok(bitmap::Bitmap::new(
            manager.display.width(),
            manager.display.height(),
        ))
    }
}
impl<'a> input::Input for SelectWifiMenu {}

impl<'a> Widget for SelectWifiMenu {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        let mut children = self.clients
            .iter_mut()
            .map(|item| item as &mut Widget)
            .collect::<Vec<_>>();
        children.push(&mut self.ap as &mut Widget);
        children.push(&mut self.back as &mut Widget);
        children
    }

    fn children(&self) -> Vec<&Widget> {
        let mut children = self.clients
            .iter()
            .map(|item| item as &Widget)
            .collect::<Vec<_>>();
        children.push(&self.ap as &Widget);
        children.push(&self.back as &Widget);
        children
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
    fn render(&self, _manager: &DisplayManager, window: &Window) -> error::Result<bitmap::Bitmap> {
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
    _config: config::WifiApConfig,
    active: bool,
}

impl WifiAp {
    fn new(disp: &mut DisplayManager, config: config::WifiApConfig) -> error::Result<WifiAp> {
        Ok(WifiAp {
            windowid: disp.add_child(Position::Normal)?,
            _config: config,
            active: false,
        })
    }
}

impl render::Render for WifiAp {
    fn render(&self, _manager: &DisplayManager, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(
            &bitmap::with_border(
                font::render_text("Activate AP"),
                bitmap::BorderStyle::Top,
                2,
            ),
            (12, 0),
        );
        if self.active {
            base.blit(&bitmap::Bitmap::from_slice(font::SQUARE), (6, 0));
        }
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for WifiAp {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => {
                //TODO: pull values from config into hostapd.conf
                utils::run_check_output("hostapd", &["-B", "/etc/hostapd.conf"])?;
                self.active = true;
                Ok((true, vec![]))
            }
            _ => Ok((false, vec![])),
        }
    }

    fn do_action(
        &mut self,
        _disp: &mut DisplayManager,
        _action: &action::Action,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        Ok((false, vec![]))
    }
}

impl Widget for WifiAp {
    fn windowid(&self) -> WindowId {
        self.windowid
    }
}
