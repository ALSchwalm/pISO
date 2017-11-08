#include "piso.hpp"
#include "bitmap.hpp"
#include "config.hpp"
#include "lvmwrapper.hpp"
#include <algorithm>
#include <iostream>

pISO::pISO() : m_selection{m_list_items.end()} {
  rebuild_drives_from_volumes();
}

bool pISO::has_selection() const { return m_selection != m_list_items.end(); }

void pISO::update_list_items() {
  piso_log("pISO: Updating menu items");
  m_list_items.clear();
  for (auto &drive : m_drives) {
    m_list_items.push_back(&drive);
  }
  m_selection = m_list_items.begin();
}

void pISO::rebuild_drives_from_volumes() {
  piso_log("Rebuilding VirtualDrives from lvm volumes");
  m_drives.clear();

  auto lvs = lvm_lvs_report();
  for (const auto &volume : lvs) {

    // Only if the logical volume is (V)irtual (to ignore metadata, etc)
    if (volume["lv_attr"].asString()[0] == 'V') {
      piso_log("Found volume ", volume["lv_name"]);
      m_drives.push_back(VirtualDrive(volume["lv_name"].asString()));
    }
  }
  update_list_items();
}

const VirtualDrive &pISO::add_drive(uint64_t size) {
  piso_log("Adding new drive with size=", size);

  auto name = "volume" + std::to_string(m_drives.size());

  lvm_run("lvcreate -V ", size, "B -T ", VOLUME_GROUP_NAME, "/", THINPOOL_NAME,
          " -n ", name);
  m_drives.emplace_back(name);

  // // TODO: create partition table on new volume

  m_drives.back().mount_external();

  update_list_items();
  return m_drives.back();
}

void pISO::remove_drive(const VirtualDrive &drive) {
  piso_log("Removing drive ", drive.name());
  auto drive_iter = std::find(m_drives.begin(), m_drives.end(), drive);
  if (drive_iter == m_drives.end()) {
    piso_log("Warning: drive not found");
    return;
  }

  lvm_run("lvremove ", VOLUME_GROUP_NAME, "/", drive.name(), " -y");

  m_drives.erase(drive_iter);
  update_list_items();
}

float pISO::percent_used() const {
  // The percent used for the whole drive is really the percent of the
  // thin pool. The volume group will always be full (with the thinpool).
  auto lvs = lvm_lvs_report();
  for (const auto &volume : lvs) {
    if (volume["lv_name"].asString() == THINPOOL_NAME) {
      return std::stof(volume["data_percent"].asString());
    }
  }
  piso_error("pISO: unable to locate thinpool");
}

bool pISO::on_select() {
  piso_log("pISO::on_select()");
  if (has_selection()) {
    return (*m_selection)->on_select();
  } else {
    return false;
  }
}

bool pISO::on_next() {
  piso_log("pISO::on_next()");
  if (has_selection()) {
    if (!(*m_selection)->on_next()) {
      m_selection++;
    }
    return true;
  } else {
    return false;
  }
}

bool pISO::on_prev() {
  piso_log("pISO::on_prev()");
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

Bitmap pISO::render() const {
  piso_log("pISO::render()");
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
