
#include "virtualdrive.hpp"
#include "config.hpp"
#include "error.hpp"
#include "font.hpp"
#include <errno.h>
#include <sys/stat.h>
#include <sys/types.h>

bool VirtualDriveHeading::on_select() {
  multitool_log("VirtualDriveHeading::on_select()");
  if (m_vdrive.mount_state() == VirtualDrive::MountState::UNMOUNTED) {
    m_vdrive.mount_internal();
  } else {
    m_vdrive.unmount_internal();
  }
  return true;
}

Bitmap VirtualDriveHeading::render() const {
  multitool_log("VirtualDriveHeading::render()");
  return render_text(m_vdrive.name());
}

VirtualDrive::VirtualDrive(const std::string &volume_name)
    : m_volume_name{volume_name}, m_heading{*this}, m_selection{
                                                        m_list_items.end()} {
  m_uuid = lvm_lvs_volume_value("lv_uuid", volume_name);
  auto sizestr =
      lvm_lvs_report("lv_size --units B", volume_name)["lv_size"].asString();
  m_size = std::stoull(sizestr);
}

VirtualDrive::VirtualDrive(VirtualDrive &&other)
    : m_volume_name{other.m_volume_name}, m_uuid{other.m_uuid},
      m_size{other.m_size}, m_isos{other.m_isos},
      m_mount_state{other.m_mount_state}, m_heading{*this} {
  update_list_items();
}

VirtualDrive &VirtualDrive::operator=(VirtualDrive &&other) {
  m_volume_name = std::move(other.m_volume_name);
  m_uuid = std::move(other.m_uuid);
  m_size = std::move(other.m_size);
  m_isos = std::move(other.m_isos);
  m_mount_state = std::move(other.m_mount_state);

  update_list_items();
  return *this;
}

bool VirtualDrive::mount_internal() {
  multitool_log("VirtualDrive::mount_internal()");

  if (m_mount_state != MountState::UNMOUNTED) {
    multitool_log("Drive is not unmounted");
    return false;
  }

  auto base_mount = config_getenv("MULTITOOL_BASE_MOUNT");
  auto path = base_mount + "/" + name();
  if (mkdir(path.c_str(), 0777) == -1 && errno != EEXIST) {
    multitool_error("Cannot create path: ", path);
  }

  auto scripts_path = config_getenv("MULTITOOL_SCRIPTS_PATH");
  auto vdrive_script = scripts_path + "/vdrive.sh";

  run_command("sh ", vdrive_script, " mount ", name(), " ", path);

  update_list_items();
  return true;
}

bool VirtualDrive::unmount_internal() {
  multitool_log("VirtualDrive::unmount_internal()");

  if (m_mount_state != MountState::INTERNAL) {
    multitool_log("Drive is not mounted internal");
    return false;
  }

  auto base_mount = config_getenv("MULTITOOL_BASE_MOUNT");
  auto path = base_mount + "/" + name();

  auto scripts_path = config_getenv("MULTITOOL_SCRIPTS_PATH");
  auto vdrive_script = scripts_path + "/vdrive.sh";

  run_command("sh ", vdrive_script, " unmount ", path);
  m_mount_state = MountState::UNMOUNTED;
  return true;
}

bool VirtualDrive::has_selection() const {
  return m_selection != m_list_items.end();
}

void VirtualDrive::update_list_items() {
  multitool_log("VirtualDrive: Updating menu items");
  m_list_items.clear();
  m_list_items.push_back(&m_heading);
  for (auto &iso : m_isos) {
    m_list_items.push_back(&iso);
  }
  m_selection = m_list_items.begin();
}

float VirtualDrive::percent_used() const {
  return std::stof(lvm_lvs_volume_value("data_percent", m_volume_name));
}

bool VirtualDrive::on_select() {
  multitool_log("VirtualDrive::on_select()");
  if (has_selection()) {
    return (*m_selection)->on_select();
  } else {
    return false;
  }
}

bool VirtualDrive::on_next() {
  multitool_log("VirtualDrive::on_next()");
  if (has_selection()) {
    if (!(*m_selection)->on_next()) {
      m_selection++;
    }
    return true;
  } else {
    return false;
  }
}

bool VirtualDrive::on_prev() {
  multitool_log("VirtualDrive::on_prev()");
  if (has_selection()) {
    if (!(*m_selection)->on_prev()) {
      if (m_selection != m_list_items.begin()) {
        m_selection--;
      } else {
        return false;
      }
    }
    return true;
  } else {
    return false;
  }
}

Bitmap VirtualDrive::render() const {
  multitool_log("VirtualDrive::render()");
  auto bitmap = m_heading.render();
  for (const auto &iso : m_isos) {
    auto iso_bitmap = iso.render();
    auto old_height = bitmap.height();
    bitmap.expand_height(iso_bitmap.height());
    if (iso_bitmap.width() > bitmap.width()) {
      bitmap.expand_width(iso_bitmap.width() - bitmap.width());
    }
    bitmap.blit(iso_bitmap, {0, old_height});
  }
  return bitmap;
}
