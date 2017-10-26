#include "multitool.hpp"
#include "bitmap.hpp"
#include <algorithm>
#include <iostream>

static constexpr char VOLUME_GROUP_NAME[] = "VolGroup00";
static constexpr char THINPOOL_NAME[] = "thinpool";

Multitool::Multitool() : m_selection{m_list_items.end()} {
  m_lvm = lvm_init(NULL);
  if (m_lvm == NULL) {
    multitool_error("lvm_init()", lvm_errmsg(m_lvm));
  }

  if (lvm_scan(m_lvm) == -1) {
    multitool_error("lvm_scan()", lvm_errmsg(m_lvm));
  }

  m_volgroup = lvm_vg_open(m_lvm, VOLUME_GROUP_NAME, "w", 0);
  if (m_volgroup == NULL) {
    multitool_error("lvm_vg_open()", lvm_errmsg(m_lvm));
  }

  m_thinpool = lvm_lv_from_name(m_volgroup, THINPOOL_NAME);
  if (m_thinpool == NULL) {
    multitool_error("lvm_lv_from_name() could not locate 'thinpool'");
  }

  rebuild_drives_from_volumes();
}

Multitool::~Multitool() {
  lvm_vg_close(m_volgroup);
  lvm_quit(m_lvm);
}

bool Multitool::has_selection() const {
  return m_selection != m_list_items.end();
}

void Multitool::update_list_items() {
  multitool_log("Updating menu items");
  m_list_items.clear();
  for (auto &drive : m_drives) {
    m_list_items.push_back(&drive);
  }
  m_selection = m_list_items.begin();
}

void Multitool::rebuild_drives_from_volumes() {
  multitool_log("Rebuilding VirtualDrives from lvm volumes");
  m_drives.clear();

  // Create a virtual drive for each logical volume in the group
  auto logical_volumes = lvm_vg_list_lvs(m_volgroup);
  struct lvm_lv_list *lv_list;
  dm_list_iterate_items(lv_list, logical_volumes) {
    auto lv = lv_list->lv;
    auto attr = lvm_lv_get_attr(lv);

    // Only if the logical volume is (V)irtual (to ignore metadata, etc)
    if (attr[0] == 'V') {
      multitool_log("Found volume ", lvm_lv_get_name(lv));
      m_drives.push_back(VirtualDrive(lv));
    }
  }

  update_list_items();
}

const VirtualDrive &Multitool::add_drive(uint64_t size) {
  multitool_log("Adding new drive with size=", size);

  auto name = "volume" + std::to_string(m_drives.size());
  auto volume_params =
      lvm_lv_params_create_thin(m_volgroup, THINPOOL_NAME, name.c_str(), size);

  if (volume_params == NULL) {
    multitool_error("lvm_lv_params_create_thin()", lvm_errmsg(m_lvm));
  }

  auto lv = lvm_lv_create(volume_params);
  if (lv == NULL) {
    multitool_error("lvm_lv_create()", lvm_errmsg(m_lvm));
  }

  if (lvm_lv_activate(lv) == -1) {
    multitool_error("lvm_lv_activate()", lvm_errmsg(m_lvm));
  }

  // TODO: not sure if I need to do this
  if (lvm_vg_write(m_volgroup) == -1) {
    multitool_error("lvm_vg_write()", lvm_errmsg(m_lvm));
  }

  m_drives.emplace_back(lv);

  // TODO: create partition table on new volume

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

  if (lvm_vg_remove_lv(drive.volume()) == -1) {
    multitool_error("lvm_vg_remove_lv()", lvm_errmsg(m_lvm));
  }
  m_drives.erase(drive_iter);
  update_list_items();
}

float Multitool::percent_used() const {
  auto prop = lvm_lv_get_property(m_thinpool, "data_percent");
  if (!prop.is_valid) {
    multitool_error("data_percent is not a valid property");
  }
  return lvm_percent_to_float(prop.value.integer);
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
    auto old_height = bitmap.height();
    bitmap.expand_height(drive_bitmap.height());
    if (drive_bitmap.width() > bitmap.width()) {
      bitmap.expand_width(drive_bitmap.width() - bitmap.width());
    }
    bitmap.blit(drive_bitmap, {0, old_height});
  }
  return bitmap;
}
