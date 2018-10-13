use action;
use bitmap;
use controller;
use display;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error;
use font;
use input;
use utils;
use render;
use state;

#[derive(Debug, Eq, PartialEq)]
pub enum PiVersion {
    Zero12,
    Zero13,
    Zero131,
    ZeroW11,

    Unknown,
}

impl PiVersion {
    pub fn has_wifi(&self) -> bool {
        match self {
            // Just assume an unknown version supports wifi
            &PiVersion::ZeroW11 | &PiVersion::Unknown => true,
            _ => false,
        }
    }
}

pub fn read_version() -> error::Result<PiVersion> {
    let version = utils::run_check_output(
        "awk",
        &[
            "/^Revision/ {sub(\"^1000\", \"\", $3); print $3}",
            "/proc/cpuinfo",
        ],
    )?;

    match version.trim() {
        "900092" => Ok(PiVersion::Zero12),
        "900093" => Ok(PiVersion::Zero13),
        "920093" => Ok(PiVersion::Zero131),
        "9000c1" => Ok(PiVersion::ZeroW11),
        _ => Ok(PiVersion::Unknown),
    }
}

static PISO_VERSION: &'static str = include_str!("../VERSION");

enum VersionState {
    Unselected,
    Selected(OpenVersionMenu),
}

pub struct VersionMenu {
    pub window: WindowId,
    state: VersionState,
}

impl VersionMenu {
    pub fn new(disp: &mut DisplayManager) -> error::Result<VersionMenu> {
        Ok(VersionMenu {
            window: disp.add_child(Position::Normal)?,
            state: VersionState::Unselected,
        })
    }
}

impl render::Render for VersionMenu {
    fn render(&self, _manager: &DisplayManager, window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(10, 1);
        base.blit(&font::render_text("Version"), (16, 0));
        if window.focus {
            base.blit(&bitmap::Bitmap::from_slice(font::ARROW), (0, 0));
        }
        Ok(base)
    }
}

impl input::Input for VersionMenu {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => Ok((true, vec![action::Action::OpenVersion])),
            _ => Ok((false, vec![])),
        }
    }

    fn do_action(
        &mut self,
        disp: &mut DisplayManager,
        action: &action::Action,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *action {
            action::Action::OpenVersion => {
                let menu = OpenVersionMenu::new(disp)?;
                disp.shift_focus(&menu);
                self.state = VersionState::Selected(menu);
                Ok((true, vec![]))
            }
            action::Action::CloseVersion => {
                disp.shift_focus(self);
                self.state = VersionState::Unselected;
                Ok((true, vec![]))
            }
            _ => Ok((false, vec![])),
        }
    }
}

impl state::State for VersionMenu {}

impl Widget for VersionMenu {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        match self.state {
            VersionState::Selected(ref mut widget) => vec![widget],
            VersionState::Unselected => vec![],
        }
    }

    fn children(&self) -> Vec<&Widget> {
        match self.state {
            VersionState::Selected(ref widget) => vec![widget],
            VersionState::Unselected => vec![],
        }
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}

struct OpenVersionMenu {
    pub window: WindowId,
}

impl OpenVersionMenu {
    fn new(disp: &mut DisplayManager) -> error::Result<OpenVersionMenu> {
        Ok(OpenVersionMenu {
            window: disp.add_child(Position::Fixed(0, 0))?,
        })
    }
}

impl render::Render for OpenVersionMenu {
    fn render(&self, _manager: &DisplayManager, _window: &Window) -> error::Result<bitmap::Bitmap> {
        let mut base = bitmap::Bitmap::new(display::DISPLAY_WIDTH, display::DISPLAY_HEIGHT);
        base.blit(&font::render_text(format!("OS Version: {}", PISO_VERSION)), (6, 0));
        Ok(base)
    }
}

impl input::Input for OpenVersionMenu {
    fn on_event(
        &mut self,
        event: &controller::Event,
    ) -> error::Result<(bool, Vec<action::Action>)> {
        match *event {
            controller::Event::Select => Ok((true, vec![action::Action::CloseVersion])),
            _ => Ok((false, vec![])),
        }
    }
}

impl state::State for OpenVersionMenu {}

impl Widget for OpenVersionMenu {
    fn windowid(&self) -> WindowId {
        self.window
    }
}
