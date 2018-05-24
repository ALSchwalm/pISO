use action;
use bitmap;
use buttons;
use config;
use controller;
use display;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error;
use error::ResultExt;
use font;
use input;
use render;
use utils;

use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::{thread, time};

const HOSTAPD_CONF: &'static str = "/etc/hostapd.conf";
const HOSTAPD_TMP_CONF: &'static str = "/tmp/hostapd.conf";
const WPA_SUPPLICANT_CONF: &'static str = "/etc/wpa_supplicant.conf";
const WPA_SUPPLICANT_TMP_CONF: &'static str = "/tmp/wpa_supplicant.conf";
const SMB_CONF: &'static str = "/etc/samba/smb.conf";
const SMB_TMP_CONF: &'static str = "/tmp/smb.conf";
const PURE_FTPD_CONF: &'static str = "/etc/pure-ftpd.conf";

#[derive(PartialEq)]
enum WifiState {
    Ap,
    Client(usize),
    Inactive,
    Uninitialized,
}

struct WifiManager {
    config: config::Config,
    pub state: WifiState,
}

impl WifiManager {
    fn new(config: config::Config) -> Arc<Mutex<WifiManager>> {
        Arc::new(Mutex::new(WifiManager {
            config: config,
            state: WifiState::Uninitialized,
        }))
    }

    fn enable_wifi(&mut self) -> error::Result<()> {
        if self.state != WifiState::Uninitialized {
            return Ok(());
        }

        // Now load the driver (do this here for faster boot)
        utils::run_check_output("modprobe", &["brcmfmac"])?;

        fs::copy(HOSTAPD_CONF, HOSTAPD_TMP_CONF)?;

        let passphrase = format!("wpa_passphrase={}\n", self.config.wifi.ap.password);
        let ssid = format!("ssid={}\n", self.config.wifi.ap.ssid);

        let mut hostapd = fs::OpenOptions::new().append(true).open(HOSTAPD_TMP_CONF)?;
        hostapd.write_all(passphrase.as_bytes())?;
        hostapd.write_all(ssid.as_bytes())?;

        fs::copy(WPA_SUPPLICANT_CONF, WPA_SUPPLICANT_TMP_CONF)?;
        let mut wpa_supplicant = fs::OpenOptions::new()
            .append(true)
            .open(WPA_SUPPLICANT_TMP_CONF)?;

        for client in self.config.wifi.client.iter() {
            let mut output =
                utils::run_check_output("wpa_passphrase", &[&client.ssid, &client.password])?;
            // Remove the trailing newline and '}'
            output.pop();
            output.pop();

            // Disable all networks by default
            output += "\tdisabled=1\n}\n";

            wpa_supplicant.write_all(output.as_bytes())?;
        }

        // TODO: determine why we need to sleep here
        thread::sleep(time::Duration::from_secs(1));

        fs::copy(SMB_CONF, SMB_TMP_CONF)?;
        utils::run_check_output("smbd", &["-D", "-s", SMB_TMP_CONF])?;
        utils::run_check_output("nmbd", &["-D", "-s", SMB_TMP_CONF])?;

        utils::run_check_output("pure-ftpd", &[PURE_FTPD_CONF])?;

        self.state = WifiState::Inactive;
        Ok(())
    }

    fn activate_host(&mut self) -> error::Result<()> {
        match self.state {
            WifiState::Ap => (),
            WifiState::Client(_) => {
                self.deactivate_client()?;
                self.activate_host()?;
            }
            WifiState::Inactive | WifiState::Uninitialized => {
                self.enable_wifi()?;
                utils::run_check_output("ip", &["addr", "add", "dev", "wlan0", "10.55.55.1/24"])?;
                utils::run_check_output("hostapd", &["-B", HOSTAPD_TMP_CONF])?;
                self.state = WifiState::Ap;
            }
        }
        Ok(())
    }

    fn deactivate_host(&mut self) -> error::Result<()> {
        match self.state {
            WifiState::Ap => {
                utils::run_check_output("killall", &["hostapd"])?;
                self.state = WifiState::Inactive;
            }
            WifiState::Client(_) => {
                return Err("Attempt to deactivate host while in client mode".into())
            }
            _ => (),
        }
        Ok(())
    }

    fn toggle_host(&mut self) -> error::Result<()> {
        match self.state {
            WifiState::Ap => self.deactivate_host(),
            _ => self.activate_host(),
        }
    }

    fn activate_client(&mut self, network_num: usize) -> error::Result<()> {
        match self.state {
            WifiState::Client(_) => {
                self.deactivate_client()?;
                self.activate_client(network_num)?;
            }
            WifiState::Ap => {
                self.deactivate_host()?;
                self.activate_client(network_num)?;
            }
            WifiState::Inactive | WifiState::Uninitialized => {
                self.enable_wifi()?;

                // Kill any existing WPA supplicant (this will only happen if a
                // previous activate attempt errored)
                let _ = utils::run_check_output("killall", &["wpa_supplicant"]);

                utils::run_check_output(
                    "wpa_supplicant",
                    &["-B", "-i", "wlan0", "-c", WPA_SUPPLICANT_TMP_CONF],
                )?;
                utils::run_check_output(
                    "/opt/piso_scripts/wifi_client.sh",
                    &[network_num.to_string()],
                )?;

                utils::run_check_output("udhcpc", &["-i", "wlan0", "-n"])
                    .chain_err(|| "Failed to obtain dhcp lease")?;
                self.state = WifiState::Client(network_num);
            }
        };
        Ok(())
    }

    fn deactivate_client(&mut self) -> error::Result<()> {
        match self.state {
            WifiState::Client(_) => {
                // Suppress failure if it isn't running
                let _ = utils::run_check_output("killall", &["wpa_supplicant"]);
                self.state = WifiState::Inactive;
            }
            WifiState::Ap => return Err("Attempt to deactivate client while in host mode".into()),
            WifiState::Inactive | WifiState::Uninitialized => (),
        };
        Ok(())
    }

    fn toggle_client(&mut self, network_num: usize) -> error::Result<()> {
        match self.state {
            WifiState::Client(num) => {
                if num == network_num {
                    self.deactivate_client()
                } else {
                    self.activate_client(network_num)
                }
            }
            _ => self.activate_client(network_num),
        }
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
            .enumerate()
            .map(|(id, config)| {
                WifiClient::new(disp, config.clone(), manager.clone(), id + 1)
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
    manager: Arc<Mutex<WifiManager>>,
    menu: Option<WifiConnectedMenu>,
    id: usize,
}

impl WifiClient {
    fn new(
        disp: &mut DisplayManager,
        config: config::WifiClientNetworkConfig,
        manager: Arc<Mutex<WifiManager>>,
        id: usize,
    ) -> error::Result<WifiClient> {
        Ok(WifiClient {
            windowid: disp.add_child(Position::Normal)?,
            config: config,
            manager: manager,
            menu: None,
            id: id,
        })
    }
}

impl render::Render for WifiClient {
    fn render(&self, _manager: &DisplayManager, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(&font::render_text(&self.config.ssid), (12, 0));
        match self.manager.lock()?.state {
            WifiState::Client(id) => {
                if id == self.id {
                    base.blit(&bitmap::Bitmap::from_slice(font::SQUARE), (6, 0));
                }
            }
            _ => (),
        }
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for WifiClient {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => {
                let message = match self.manager.lock()?.toggle_client(self.id) {
                    Ok(()) => format!(
                        "Connected: {}",
                        utils::run_check_output(
                            "/opt/piso_scripts/wifi_address.sh",
                            &[] as &[&str],
                        )?.trim_right()
                    ),
                    //TODO: this text should wrap
                    Err(e) => format!("Failed: {}", e.description()),
                };
                Ok((true, vec![action::Action::OpenWifiConnectedMenu(message)]))
            }
            _ => Ok((false, vec![])),
        }
    }

    fn do_action(
        &mut self,
        disp: &mut DisplayManager,
        action: &action::Action,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match action {
            &action::Action::OpenWifiConnectedMenu(ref msg) => {
                let menu = WifiConnectedMenu::new(disp, msg.clone())?;
                disp.shift_focus(&menu);
                self.menu = Some(menu);
                Ok((true, vec![]))
            }
            &action::Action::CloseWifiConnectedMenu => {
                self.menu = None;
                disp.shift_focus(self);
                Ok((true, vec![]))
            }
            _ => Ok((false, vec![])),
        }
    }
}

impl Widget for WifiClient {
    fn windowid(&self) -> WindowId {
        self.windowid
    }

    fn mut_children(&mut self) -> Vec<&mut Widget> {
        match self.menu {
            Some(ref mut menu) => vec![menu],
            None => vec![],
        }
    }

    fn children(&self) -> Vec<&Widget> {
        match self.menu {
            Some(ref menu) => vec![menu],
            None => vec![],
        }
    }
}

pub struct WifiConnectedMenu {
    pub windowid: WindowId,
    message: String,
}

impl WifiConnectedMenu {
    fn new(disp: &mut DisplayManager, message: String) -> error::Result<WifiConnectedMenu> {
        Ok(WifiConnectedMenu {
            windowid: disp.add_child(Position::Fixed(0, 0))?,
            message: message,
        })
    }
}

impl Widget for WifiConnectedMenu {
    fn windowid(&self) -> WindowId {
        self.windowid
    }
}

impl render::Render for WifiConnectedMenu {
    fn render(&self, _manager: &DisplayManager, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(display::DISPLAY_WIDTH, display::DISPLAY_HEIGHT);
        base.blit(&font::render_text(&self.message), (0, 0));
        base.blit(&font::render_text("Ok"), (10, 20));
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 20));
        }
        Ok(base)
    }
}

impl input::Input for WifiConnectedMenu {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => Ok((true, vec![action::Action::CloseWifiConnectedMenu])),
            _ => Ok((false, vec![])),
        }
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
            manager: manager,
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
                self.manager.lock()?.toggle_host()?;
                Ok((true, vec![]))
            }
            _ => Ok((false, vec![])),
        }
    }
}

impl Widget for WifiAp {
    fn windowid(&self) -> WindowId {
        self.windowid
    }
}
