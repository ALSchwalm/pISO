use lvm;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Action {
    CreateDrive(lvm::LogicalVolume),
    ToggleVDriveMount(u32),
    ToggleIsoMount(u32),

    OpenSizeMenu,
    CloseSizeMenu,
    IncDriveSize,
    DecDriveSize,

    OpenFormatMenu,
    CloseFormatMenu,
    FormatDrive,

    OpenWifiMenu,
    CloseWifiMenu,

    OpenWifiConnectedMenu(String),
    CloseWifiConnectedMenu,
}
