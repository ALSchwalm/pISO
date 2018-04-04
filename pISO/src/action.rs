use lvm;

#[derive(Debug)]
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
    NextFormat,
    PrevFormat,
}
