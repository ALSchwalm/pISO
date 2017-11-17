
#include "virtualdrive.hpp"
#include "config.hpp"
#include "error.hpp"
#include "font.hpp"
#include <errno.h>
#include <sys/stat.h>
#include <sys/types.h>

bool VirtualDriveHeading::on_select() {
  piso_log("VirtualDriveHeading::on_select()");
  switch (m_vdrive.mount_state()) {
  case VirtualDrive::MountState::UNMOUNTED:
    m_vdrive.mount_external();
    break;
  case VirtualDrive::MountState::EXTERNAL:
    m_vdrive.unmount_external();
    m_vdrive.mount_internal();
    break;
  case VirtualDrive::MountState::INTERNAL:
    m_vdrive.unmount_internal();
    m_vdrive.mount_external();
    break;
  }
  return true;
}

Bitmap VirtualDriveHeading::render() const {
  piso_log("VirtualDriveHeading::render()");
  auto text = render_text(m_vdrive.name());
  if (m_focused) {
    Bitmap with_select(text.width() + 7, text.height());
    with_select.blit(text, {7, 0});
    with_select.blit(selector, {0, 0});
    return with_select;
  } else {
    return text;
  }
}

VirtualDrive::VirtualDrive(const std::string &volume_name)
    : m_volume_name{volume_name}, m_heading{*this}, m_selection{
                                                        m_list_items.end()} {
  m_uuid = lvm_lvs_volume_value("lv_uuid", volume_name);
  auto sizestr =
      lvm_lvs_report("lv_size --units B", volume_name)["lv_size"].asString();
  m_size = std::stoull(sizestr);
  update_list_items();
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
  piso_log("VirtualDrive::mount_internal()");

  if (m_mount_state != MountState::UNMOUNTED) {
    piso_log("Drive is not unmounted");
    return false;
  }

  auto base_mount = config_getenv("PISO_BASE_MOUNT");
  auto path = base_mount + "/" + name();
  if (mkdir(path.c_str(), 0777) == -1 && errno != EEXIST) {
    piso_error("Cannot create path: ", path);
  }

  auto scripts_path = config_getenv("PISO_SCRIPTS_PATH");
  auto vdrive_script = scripts_path + "/vdrive.sh";

  run_command("sh ", vdrive_script, " mount-internal ", name(), " ", path);
  update_list_items();
  m_mount_state = MountState::INTERNAL;
  return true;
}

bool VirtualDrive::unmount_internal() {
  piso_log("VirtualDrive::unmount_internal()");

  if (m_mount_state != MountState::INTERNAL) {
    piso_log("Drive is not mounted internal");
    return false;
  }

  auto base_mount = config_getenv("PISO_BASE_MOUNT");
  auto path = base_mount + "/" + name();

  auto scripts_path = config_getenv("PISO_SCRIPTS_PATH");
  auto vdrive_script = scripts_path + "/vdrive.sh";

  run_command("sh ", vdrive_script, " unmount-internal ", path);
  update_list_items();
  m_mount_state = MountState::UNMOUNTED;
  return true;
}

bool VirtualDrive::mount_external() {
  piso_log("VirtualDrive::mount_external()");

  if (m_mount_state != MountState::UNMOUNTED) {
    piso_log("Drive is mounted");
    return false;
  }

  auto scripts_path = config_getenv("PISO_SCRIPTS_PATH");
  auto vdrive_script = scripts_path + "/vdrive.sh";

  run_command("sh ", vdrive_script, " mount-external ", name());
  update_list_items();
  m_mount_state = MountState::EXTERNAL;
  return true;
}

bool VirtualDrive::unmount_external() {
  piso_log("VirtualDrive::unmount_external()");

  if (m_mount_state != MountState::EXTERNAL) {
    piso_log("Drive is not mounted external");
    return false;
  }

  auto scripts_path = config_getenv("PISO_SCRIPTS_PATH");
  auto vdrive_script = scripts_path + "/vdrive.sh";

  run_command("sh ", vdrive_script, " unmount-external ", name());
  update_list_items();
  m_mount_state = MountState::UNMOUNTED;
  return true;
}

bool VirtualDrive::has_selection() const {
  return m_selection != m_list_items.end();
}

void VirtualDrive::update_list_items() {
  piso_log("VirtualDrive: Updating menu items");
  if (has_selection()) {
    (*m_selection)->on_lose_focus();
  }
  m_list_items.clear();
  m_list_items.push_back(&m_heading);
  for (auto &iso : m_isos) {
    m_list_items.push_back(&iso);
  }
  m_selection = m_list_items.begin();
  if (has_selection()) { // FIXME: only do this if something lost focus?
    (*m_selection)->on_focus();
  }
}

float VirtualDrive::percent_used() const {
  return std::stof(lvm_lvs_volume_value("data_percent", m_volume_name));
}

bool VirtualDrive::on_focus() {
  GUIEventHandler::on_focus();
  if (has_selection()) {
    (*m_selection)->on_focus();
  }
}

bool VirtualDrive::on_lose_focus() {
  GUIEventHandler::on_lose_focus();
  if (has_selection()) {
    (*m_selection)->on_lose_focus();
  }
}

bool VirtualDrive::on_select() {
  piso_log("VirtualDrive::on_select()");
  if (has_selection()) {
    return (*m_selection)->on_select();
  } else {
    return false;
  }
}

bool VirtualDrive::on_next() {
  piso_log("VirtualDrive::on_next()");
  if (has_selection()) {
    if (!(*m_selection)->on_next()) {
      if (std::next(m_selection) != m_list_items.end()) {
        (*m_selection)->on_lose_focus();
        m_selection++;
        (*m_selection)->on_focus();
      } else {
        return false;
      }
    }
    return true;
  } else {
    return false;
  }
}

bool VirtualDrive::on_prev() {
  piso_log("VirtualDrive::on_prev()");
  if (has_selection()) {
    if (!(*m_selection)->on_prev()) {
      if (m_selection != m_list_items.begin()) {
        (*m_selection)->on_lose_focus();
        m_selection--;
        if (has_selection()) {
          (*m_selection)->on_focus();
        }
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
  piso_log("VirtualDrive::render()");
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
