#include "piso.hpp"
#include "bitmap.hpp"
#include "config.hpp"
#include "display.hpp"
#include "font.hpp"
#include "lvmwrapper.hpp"

#include <algorithm>
#include <cmath>
#include <iomanip>
#include <iostream>
#include <linux/usb/ch9.h> // For USB_CLASS_PER_INTERFACE
#include <sstream>

bool NewDriveItem::on_select() {
  piso_log("NewDriveItem::on_select()");
  if (m_selecting_size) {
    unsigned long long new_size = m_current_percent / 100.0 * m_piso.size();
    // sizes must be a multiple of 512
    m_piso.add_drive(((new_size + 512 - 1) / 512) * 512);
    m_selecting_size = false;
    m_current_percent = 100;
  } else {
    m_selecting_size = true;
  }
  return true;
}

bool NewDriveItem::on_next() {
  piso_log("NewDriveItem::on_next()");
  if (!m_selecting_size) {
    return false;
  } else {
    m_current_percent += 10;
    return true;
  }
}

bool NewDriveItem::on_prev() {
  piso_log("NewDriveItem::on_prev()");
  if (!m_selecting_size) {
    return false;
  } else {
    m_current_percent -= 10;
    if (m_current_percent < 0) {
      m_current_percent = 0;
    }
    return true;
  }
}

std::pair<Bitmap, GUIRenderable::RenderMode> NewDriveItem::render() const {
  piso_log("NewDriveItem::render()");
  if (!m_selecting_size) {
    auto text = render_text("Add new drive");

    Bitmap indented(text.width() + MENU_INDENT, text.height());
    indented.blit(text, {MENU_INDENT, 0});
    if (m_focused) {
      indented.blit(selector, {0, 0});
      return {indented, GUIRenderable::RenderMode::NORMAL};
    } else {
      return {indented, GUIRenderable::RenderMode::NORMAL};
    }
  } else {
    Bitmap disp(Display::width, Display::height);
    auto text = render_text("New drive capacity:");
    disp.blit(text, {0, 0});

    double bytes_per_gb = 1024 * 1024 * 1024;
    std::stringstream ss;
    ss << m_current_percent << "%";
    ss << " (" << std::fixed << std::setprecision(1)
       << (m_piso.size() / bytes_per_gb * m_current_percent / 100) << "GB)";
    auto size_text = render_text(ss.str());

    // TODO: less arbitrary position
    disp.blit(size_text, {15, 25});

    return {disp, GUIRenderable::RenderMode::FULLSCREEN};
  }
}

pISO::pISO() : m_newdrive(*this) {
  rebuild_drives_from_volumes();
  // init_usbgx();
}

void pISO::init_usbgx() {
  int usbg_ret;
  usbg_state *usb_state;

  struct usbg_gadget_attrs g_attrs = {
      .bcdUSB = 0x0200, // USB2
      .bDeviceClass = USB_CLASS_PER_INTERFACE,
      .bDeviceSubClass = 0x00,
      .bDeviceProtocol = 0x00,
      .bMaxPacketSize0 = 64,
      .idVendor = VENDOR_ID,
      .idProduct = PRODUCT_ID,
      .bcdDevice = 0x0100, // v1.0.0
  };

  struct usbg_gadget_strs g_strs;
  g_strs.serial = "0000000000000000";
  g_strs.manufacturer = "Adam Schwalm & James Tate";
  g_strs.product = "pISO";

  usbg_ret = usbg_init("/sys/kernel/config", &usb_state);
  if (usbg_ret != USBG_SUCCESS) {
    piso_error("init_usbgx error:", usbg_error_name((usbg_error)usbg_ret), ": ",
               usbg_strerror((usbg_error)usbg_ret));
  }

  usbg_ret = usbg_create_gadget(usb_state, "g1", &g_attrs, &g_strs, &m_gadget);
  if (usbg_ret != USBG_SUCCESS) {
    usbg_cleanup(usb_state);
    piso_error("init_usbgx error: ", usbg_error_name((usbg_error)usbg_ret),
               ": ", usbg_strerror((usbg_error)usbg_ret));
  }

  struct usbg_config_strs c_strs = {"Config 1"};
  struct usbg_config_attrs c_attrs = {.bmAttributes = 0x80, .bMaxPower = 250};

  usbg_ret =
      usbg_create_config(m_gadget, 1, "c", &c_attrs, &c_strs, &m_usb_config);
  if (usbg_ret != USBG_SUCCESS) {
    usbg_cleanup(usb_state);
    piso_error("init_usbgx error: ", usbg_error_name((usbg_error)usbg_ret),
               ": ", usbg_strerror((usbg_error)usbg_ret));
  }
}

void pISO::update_list_items() {
  piso_log("pISO: Updating menu items");
  // m_list_items may contain invalid pointers here (if m_drives has been
  // modified, for example), so don't read from it.
  m_list_items.clear();
  for (auto &drive : m_drives) {
    m_list_items.push_back(&drive);
  }
  m_list_items.push_back(&m_newdrive);
  for (const auto &item : m_list_items) {
    item->on_lose_focus();
  }
  m_selection = m_list_items.begin();
  if (has_selection()) { // FIXME: only do this if something lost focus?
    (*m_selection)->on_focus();
  }
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

unsigned long long pISO::size() const {
  auto sizestr =
      lvm_lvs_report("lv_size --units B", THINPOOL_NAME)["lv_size"].asString();
  return std::stoull(sizestr);
}

bool pISO::on_select() {
  piso_log("pISO::on_select()");
  return GUIListItem::on_select();
}

bool pISO::on_next() {
  piso_log("pISO::on_next()");
  return GUIListItem::on_next();
}

bool pISO::on_prev() {
  piso_log("pISO::on_prev()");
  return GUIListItem::on_prev();
}

std::pair<Bitmap, GUIRenderable::RenderMode> pISO::render() const {
  piso_log("pISO::render()");
  Bitmap bitmap;
  for (const auto &item : m_list_items) {
    auto drive_bitmap = item->render();
    if (drive_bitmap.second == GUIRenderable::RenderMode::FULLSCREEN) {
      return drive_bitmap;
    }

    Bitmap shifted{drive_bitmap.first.width() + MENU_LEFT_SPACE,
                   drive_bitmap.first.height()};
    shifted.blit(drive_bitmap.first, {MENU_LEFT_SPACE, 0});

    auto old_height = bitmap.height();
    bitmap.expand_height(shifted.height());
    if (shifted.width() > bitmap.width()) {
      bitmap.expand_width(shifted.width() - bitmap.width());
    }
    bitmap.blit(shifted, {0, old_height}, true);
  }

  // TODO: Probably implement scrolling here
  Bitmap out{Display::width, Display::height};
  out.blit(bitmap, {0, 0});

  int percent_free = 100 - percent_used();
  std::string sidebar_contents = std::to_string(percent_free) + "% Free";
  auto sidebar = render_text(sidebar_contents);
  Bitmap sidebar_with_border{sidebar.width(), sidebar.height() + SIDEBAR_SPACE};
  for (auto &pixel : sidebar_with_border[0]) {
    pixel = 1; // Create the 'border' on the right
  }
  sidebar_with_border.blit(sidebar, {0, SIDEBAR_SPACE});
  sidebar_with_border = sidebar_with_border.rotate(Bitmap::Direction::Left);

  out.blit(sidebar_with_border, {out.width() - sidebar_with_border.width(), 0});
  return {out, GUIRenderable::RenderMode::NORMAL};
}
