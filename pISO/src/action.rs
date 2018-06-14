use lvm;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    CreateDrive(lvm::LogicalVolume),
    DeleteDrive(String),
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

    OpenVDriveList(u32),
    CloseVDriveList(u32),

    ToggleDriveReadOnly(String),
    ToggleDriveNonRemovable(String),
}
