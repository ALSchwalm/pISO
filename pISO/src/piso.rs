use action;
use bitmap;
use config;
use controller;
use displaymanager::{DisplayManager, Position, Widget, Window, WindowId};
use error::Result;
use input;
use lvm;
use newdrive;
use usb;
use std::sync::{Arc, Mutex};
use render;
use stats;
use vdrive;
use wifi;

pub struct PIso {
    config: config::Config,
    pub drives: Vec<vdrive::VirtualDrive>,
    newdrive: newdrive::NewDrive,
    stats: stats::Stats,
    usb: Arc<Mutex<usb::UsbGadget>>,
    vg: lvm::VolumeGroup,
    window: WindowId,
    wifi: wifi::WifiMenu,
}

impl PIso {
    pub fn new(
        disp: &mut DisplayManager,
        usb: Arc<Mutex<usb::UsbGadget>>,
        config: config::Config,
    ) -> Result<PIso> {
        let window = disp.add_child(Position::Fixed(0, 0))?;

        let vg = lvm::VolumeGroup::from_path("/dev/VolGroup00")?;
        let drives = Self::build_drives_from_vg(disp, &vg, &usb)?;
        let ndrive = newdrive::NewDrive::new(disp, usb.clone(), vg.clone())?;
        let stats = stats::Stats::new(disp, vg.clone())?;
        let wifi = wifi::WifiMenu::new(disp, &config)?;

        if drives.len() > 0 {
            // Focus the first drive
            drives.iter().next().map(|drive| {
                disp.shift_focus(drive as &Widget);
            });
        } else {
            disp.shift_focus(&ndrive);
        }

        Ok(PIso {
            config: config,
            drives: drives,
            newdrive: ndrive,
            usb: usb,
            vg: vg,
            window: window,
            stats: stats,
            wifi: wifi,
        })
    }

    fn build_drives_from_vg(
        disp: &mut DisplayManager,
        vg: &lvm::VolumeGroup,
        usb: &Arc<Mutex<usb::UsbGadget>>,
    ) -> Result<Vec<vdrive::VirtualDrive>> {
        let mut drives: Vec<vdrive::VirtualDrive> = vec![];
        for vol in vg.volumes()?.into_iter() {
            drives.push(vdrive::VirtualDrive::new(disp, usb.clone(), vol)?)
        }
        Ok(drives)
    }

    fn add_drive(
        &mut self,
        disp: &mut DisplayManager,
        volume: lvm::LogicalVolume,
    ) -> Result<&vdrive::VirtualDrive> {
        let vdrive = vdrive::VirtualDrive::new(disp, self.usb.clone(), volume)?;
        self.drives.push(vdrive);

        Ok(self.drives
            .last()
            .expect("vdrive was somehow empty after push"))
    }
}

impl render::Render for PIso {
    fn render(&self, _: &Window) -> Result<bitmap::Bitmap> {
        Ok(bitmap::Bitmap::new(0, 0))
    }
}

impl input::Input for PIso {
    fn on_event(&mut self, _: &controller::Event) -> Result<(bool, Vec<action::Action>)> {
        Ok((false, vec![]))
    }

    fn do_action(
        &mut self,
        disp: &mut DisplayManager,
        action: &action::Action,
    ) -> Result<(bool, Vec<action::Action>)> {
        match *action {
            action::Action::CreateDrive(ref volume) => {
                self.add_drive(disp, volume.clone())?;
                Ok((true, vec![]))
            }
            _ => Ok((false, vec![])),
        }
    }
}

impl Widget for PIso {
    fn mut_children(&mut self) -> Vec<&mut Widget> {
        let mut children = self.drives
            .iter_mut()
            .map(|vdrive| vdrive as &mut Widget)
            .collect::<Vec<&mut Widget>>();
        children.push(&mut self.newdrive as &mut Widget);
        children.push(&mut self.wifi as &mut Widget);
        children.push(&mut self.stats as &mut Widget);
        children
    }

    fn children(&self) -> Vec<&Widget> {
        let mut children = self.drives
            .iter()
            .map(|vdrive| vdrive as &Widget)
            .collect::<Vec<&Widget>>();
        children.push(&self.newdrive as &Widget);
        children.push(&self.wifi as &Widget);
        children.push(&self.stats as &Widget);
        children
    }

    fn windowid(&self) -> WindowId {
        self.window
    }
}
