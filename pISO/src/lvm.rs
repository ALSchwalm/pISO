use error::{ErrorKind, Result, ResultExt};
use utils;
use std::fmt::Display;
use std::str::FromStr;
use serde::de::{self, Deserialize, Deserializer};
use serde_json;
use std::path::{Path, PathBuf};
use std::process::Command;

fn from_str<'de, T, D>(deserializer: D) -> ::std::result::Result<T, D::Error>
where
    T: FromStr + Default,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.len() > 0 {
        T::from_str(&s).map_err(de::Error::custom)
    } else {
        Ok(T::default())
    }
}

fn from_str_strip_unit<'de, T, D>(deserializer: D) -> ::std::result::Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s.trim_matches('B')).map_err(de::Error::custom)
}

#[derive(Deserialize)]
pub struct LogicalVolumeReport {
    pub lv_name: String,
    pub vg_name: String,

    #[serde(deserialize_with = "from_str")]
    pub seg_count: i32,

    pub lv_attr: String,

    #[serde(deserialize_with = "from_str_strip_unit")]
    pub lv_size: u64,

    #[serde(deserialize_with = "from_str")]
    pub lv_major: i32,

    #[serde(deserialize_with = "from_str")]
    pub lv_minor: i32,

    #[serde(deserialize_with = "from_str")]
    pub lv_kernel_major: i32,

    #[serde(deserialize_with = "from_str")]
    pub lv_kernel_minor: i32,

    pub pool_lv: String,
    pub origin: String,

    #[serde(deserialize_with = "from_str")]
    pub data_percent: f64,

    #[serde(deserialize_with = "from_str")]
    pub metadata_percent: f64,
    pub move_pv: String,
    pub copy_percent: String,
    pub mirror_log: String,
    pub convert_lv: String,
    pub lv_uuid: String,
    pub lv_profile: String,
}

#[derive(Deserialize)]
pub struct VolumeGroupReport {
    pub vg_name: String,
    pub vg_attr: String,

    #[serde(deserialize_with = "from_str_strip_unit")]
    pub vg_extent_size: u64,

    #[serde(deserialize_with = "from_str")]
    pub pv_count: u32,

    #[serde(deserialize_with = "from_str")]
    pub lv_count: u32,

    #[serde(deserialize_with = "from_str")]
    pub snap_count: u32,

    #[serde(deserialize_with = "from_str_strip_unit")]
    pub vg_size: u64,

    #[serde(deserialize_with = "from_str_strip_unit")]
    pub vg_free: u64,

    pub vg_uuid: String,
    pub vg_profile: String,
}

pub fn lvs() -> Result<Vec<LogicalVolumeReport>> {
    let output = Command::new("lvs")
        .args(&["--verbose", "--report-format=json", "--units=B"])
        .output()
        .chain_err(|| "lvs failed")?;
    if !output.status.success() {
        return Err(format!("lvs failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    let mut val: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))
        .chain_err(|| "failed to parse lvs output as json")?;
    serde_json::from_value(val["report"][0]["lv"].take())
        .chain_err(|| "failed to convert lvs json to struct")
}

pub fn vgs() -> Result<Vec<VolumeGroupReport>> {
    let output = Command::new("vgs")
        .args(&["--verbose", "--report-format=json", "--units=B"])
        .output()
        .chain_err(|| "vgs failed")?;
    if !output.status.success() {
        return Err(format!("vgs failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let mut val: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))
        .chain_err(|| "failed to parse vgs output as json")?;
    serde_json::from_value(val["report"][0]["vg"].take())
        .chain_err(|| "failed to convert vgs json to struct")
}

#[derive(Clone, Debug)]
pub struct VolumeGroup {
    pub name: String,
    pub path: PathBuf,
}

impl VolumeGroup {
    pub fn from_path<P>(path: P) -> Result<VolumeGroup>
    where
        P: AsRef<Path>,
    {
        Ok(VolumeGroup {
            name: path.as_ref()
                .file_name()
                .ok_or(ErrorKind::Msg("VolumeGroup path has no filename".into()))?
                .to_string_lossy()
                .into(),
            path: path.as_ref().to_path_buf(),
        })
    }

    pub fn report(&self) -> Result<VolumeGroupReport> {
        let report = vgs()?;
        report
            .into_iter()
            .filter(|vg| vg.vg_name == self.name)
            .next()
            .ok_or("Unable to get report for vg".into())
    }

    pub fn volumes(&self) -> Result<Vec<LogicalVolume>> {
        let report = lvs()?;
        Ok(report
            .into_iter()
            .filter(|lv| lv.vg_name == self.name && !lv.lv_attr.starts_with("t"))
            .map(|lv| LogicalVolume::from_report(&self, lv))
            .collect())
    }

    pub fn pool(&self) -> Result<LogicalVolumeReport> {
        let report = lvs()?;
        report
            .into_iter()
            .filter(|lv| lv.vg_name == self.name && lv.lv_attr.starts_with("t"))
            .next()
            .ok_or("Unable to locate thin pool".into())
    }

    pub fn create_volume(&mut self, name: &str, size: u64) -> Result<LogicalVolume> {
        utils::run_check_output(
            "lvcreate",
            &[
                "-V",
                &(size.to_string() + "B"),
                "-T",
                &format!("{}/thinpool", &self.name),
                "-n",
                &name,
            ],
        )?;

        self.volumes()?
            .into_iter()
            .filter(|lv| lv.name == name)
            .next()
            .ok_or("Unable to find new volume".into())
    }

    pub fn snapshot_volume(&mut self, name: &str) -> Result<LogicalVolume> {
        let snapshot_name = format!("{}-backup", name);
        utils::run_check_output(
            "lvcreate",
            &[
                &format!("{}/{}", &self.name, name),
                "-n",
                &snapshot_name,
                "-s",
            ],
        )?;

        // Activate the snapshot and set off the skip-activation bit
        utils::run_check_output(
            "lvchange",
            &["-kn", "-ay", &format!("{}/{}", &self.name, &snapshot_name)],
        )?;

        self.volumes()?
            .into_iter()
            .filter(|lv| lv.name == snapshot_name)
            .next()
            .ok_or("Unable to find new volume".into())
    }

    pub fn delete_volume(&mut self, name: &str) -> Result<()> {
        // Note, the drive must not be currently mounted
        let drivename = format!("{}/{}", &self.name, name);
        utils::run_check_output("lvchange", &["-a", "n", &drivename])?;
        utils::run_check_output("lvremove", &[&format!("{}/{}", &self.name, name), "-y"])?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogicalVolume {
    pub name: String,
    pub path: PathBuf,
    pub size: u64
}

impl LogicalVolume {
    fn from_report(vg: &VolumeGroup, report: LogicalVolumeReport) -> LogicalVolume {
        LogicalVolume {
            path: vg.path.join(&report.lv_name).to_path_buf(),
            name: report.lv_name,
            size: report.lv_size
        }
    }
}
