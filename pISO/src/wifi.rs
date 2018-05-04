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

use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::{thread, time};

#[derive(PartialEq)]
enum WifiState {
    Ap,
    Client,
    Inactive,
}

struct WifiManager {
    config: config::Config,
    wifi_ready: bool,
    pub state: WifiState,
}

impl WifiManager {
    fn new(config: config::Config) -> Arc<Mutex<WifiManager>> {
        Arc::new(Mutex::new(WifiManager {
            config: config,
            wifi_ready: false,
            state: WifiState::Inactive,
        }))
    }

    fn enable_wifi(&mut self) -> error::Result<()> {
        if self.wifi_ready {
            return Ok(());
        }

        // Now load the driver (do this here for faster boot)
        utils::run_check_output("modprobe", &["brcmfmac"])?;

        //TODO: check that the interface actually exists
        thread::sleep(time::Duration::from_secs(2));

        fs::copy("/etc/hostapd.conf", "/tmp/hostapd.conf")?;

        let passphrase = format!("wpa_passphrase={}\n", self.config.wifi.ap.password);
        let ssid = format!("ssid={}\n", self.config.wifi.ap.ssid);

        let mut hostapd = fs::OpenOptions::new()
            .append(true)
            .open("/tmp/hostapd.conf")?;
        hostapd.write_all(passphrase.as_bytes())?;
        hostapd.write_all(ssid.as_bytes())?;

        self.wifi_ready = true;
        Ok(())
    }

    fn activate_host(&mut self) -> error::Result<()> {
        match self.state {
            WifiState::Ap => (),
            WifiState::Client => {
                //TODO: disable client
            }
            WifiState::Inactive => {
                self.enable_wifi()?;
                utils::run_check_output("hostapd", &["-B", "/tmp/hostapd.conf"])?;
                self.state = WifiState::Ap;
            }
        }
        Ok(())
    }
}

enum WifiMenuState {
    Closed,
    Open(SelectWifiMenu),
}

pub struct WifiMenu {
    state: WifiMenuState,
    config: config::Config,
    manager: Arc<Mutex<WifiManager>>,
    pub windowid: WindowId,
}

impl WifiMenu {
    pub fn new(disp: &mut DisplayManager, config: &config::Config) -> error::Result<WifiMenu> {
        Ok(WifiMenu {
            windowid: disp.add_child(Position::Normal)?,
            state: WifiMenuState::Closed,
            config: config.clone(),
            manager: WifiManager::new(config.clone()),
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
                let menu = SelectWifiMenu::new(disp, &self.config.clone(), self.manager.clone())?;
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
}

impl SelectWifiMenu {
    fn new(
        disp: &mut DisplayManager,
        config: &config::Config,
        manager: Arc<Mutex<WifiManager>>,
    ) -> error::Result<SelectWifiMenu> {
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

        let ap = WifiAp::new(disp, config.wifi.ap.clone(), manager.clone())?;

        disp.shift_focus(
            clients
                .first()
                .map(|client| client as &Widget)
                .unwrap_or(&ap),
        );
        Ok(SelectWifiMenu {
            windowid: window,
            back: buttons::back::BackButton::new(disp, action::Action::CloseWifiMenu)?,
            clients: clients,
            ap: ap,
        })
    }
}

impl render::Render for SelectWifiMenu {
    fn render(&self, manager: &DisplayManager, _window: &Window) -> error::Result<bitmap::Bitmap> {
        Ok(bitmap::Bitmap::new(
            manager.display.width(),
            manager.display.height(),
        ))
    }
}
impl input::Input for SelectWifiMenu {}

impl Widget for SelectWifiMenu {
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
    manager: Arc<Mutex<WifiManager>>,
}

impl WifiAp {
    fn new(
        disp: &mut DisplayManager,
        config: config::WifiApConfig,
        manager: Arc<Mutex<WifiManager>>,
    ) -> error::Result<WifiAp> {
        Ok(WifiAp {
            windowid: disp.add_child(Position::Normal)?,
            _config: config,
            manager: manager.clone(),
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
        if self.manager.lock()?.state == WifiState::Ap {
            base.blit(&bitmap::Bitmap::from_slice(font::SQUARE), (6, 3));
        }
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 3));
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
                self.manager.lock()?.activate_host()?;
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
