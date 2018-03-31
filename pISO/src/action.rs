#[derive(Debug)]
pub enum Action {
    CreateDrive(u64),
    ToggleVDriveMount(u32),
}
