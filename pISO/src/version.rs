use error;
use utils;

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
