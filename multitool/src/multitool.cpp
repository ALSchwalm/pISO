#include "multitool.hpp"
#include "bitmap.hpp"
#include "config.hpp"
#include "lvmwrapper.hpp"
#include <algorithm>
#include <iostream>

Multitool::Multitool() : m_selection{m_list_items.end()} {
  rebuild_drives_from_volumes();
}

bool Multitool::has_selection() const {
  return m_selection != m_list_items.end();
}

void Multitool::update_list_items() {
  multitool_log("Multitool: Updating menu items");
  m_list_items.clear();
  for (auto &drive : m_drives) {
    m_list_items.push_back(&drive);
  }
  m_selection = m_list_items.begin();
}

void Multitool::rebuild_drives_from_volumes() {
  multitool_log("Rebuilding VirtualDrives from lvm volumes");
  m_drives.clear();

  auto lvs = lvm_lvs_report();
  for (const auto &volume : lvs) {

    // Only if the logical volume is (V)irtual (to ignore metadata, etc)
    if (volume["lv_attr"].asString()[0] == 'V') {
      multitool_log("Found volume ", volume["lv_name"]);
      m_drives.push_back(VirtualDrive(volume["lv_name"].asString()));
    }
  }
  update_list_items();
}

const VirtualDrive &Multitool::add_drive(uint64_t size) {
  multitool_log("Adding new drive with size=", size);

  auto name = "volume" + std::to_string(m_drives.size());

  lvm_run("lvcreate -V ", size, "B -T ", VOLUME_GROUP_NAME, "/", THINPOOL_NAME,
          " -n ", name);
  m_drives.emplace_back(name);

  // // TODO: create partition table on new volume

  m_drives.back().mount_external();

  update_list_items();
  return m_drives.back();
}

void Multitool::remove_drive(const VirtualDrive &drive) {
  multitool_log("Removing drive ", drive.name());
  auto drive_iter = std::find(m_drives.begin(), m_drives.end(), drive);
  if (drive_iter == m_drives.end()) {
    multitool_log("Warning: drive not found");
    return;
  }

  lvm_run("lvremove ", VOLUME_GROUP_NAME, "/", drive.name(), " -y");

  m_drives.erase(drive_iter);
  update_list_items();
}

float Multitool::percent_used() const {
  // The percent used for the whole drive is really the percent of the
  // thin pool. The volume group will always be full (with the thinpool).
  auto lvs = lvm_lvs_report();
  for (const auto &volume : lvs) {
    if (volume["lv_name"].asString() == THINPOOL_NAME) {
      return std::stof(volume["data_percent"].asString());
    }
  }
  multitool_error("Multitool: unable to locate thinpool");
}

bool Multitool::on_select() {
  multitool_log("Multitool::on_select()");
  if (has_selection()) {
    return (*m_selection)->on_select();
  } else {
    return false;
  }
}

bool Multitool::on_next() {
  multitool_log("Multitool::on_next()");
  if (has_selection()) {
    if (!(*m_selection)->on_next()) {
      m_selection++;
    }
    return true;
  } else {
    return false;
  }
}

bool Multitool::on_prev() {
  multitool_log("Multitool::on_prev()");
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

Bitmap Multitool::render() const {
  multitool_log("Multitool::render()");
  Bitmap bitmap;
  for (const auto &drive : m_drives) {
    auto drive_bitmap = drive.render();
    Bitmap shifted{drive_bitmap.width() + 3, drive_bitmap.height()};
    shifted.blit(drive_bitmap, {3, 0});

    auto old_height = bitmap.height();
    bitmap.expand_height(shifted.height());
    if (shifted.width() > bitmap.width()) {
      bitmap.expand_width(shifted.width() - bitmap.width());
    }
    bitmap.blit(shifted, {0, old_height}, true);
  }
  return bitmap;
}
