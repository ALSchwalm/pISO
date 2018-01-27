#include "newdrive.hpp"
#include "bitmap.hpp"
#include "config.hpp"
#include "controller.hpp"
#include "display.hpp"
#include "error.hpp"
#include "font.hpp"
#include "piso.hpp"
#include "virtualdrive.hpp"

#include <iomanip>
#include <thread>

void FormatItem::add_new_formatted_drive(const DriveFormat &format) {
  piso_log("FormatItem::add_new_formatted_drive()");
  m_formatting = true;
  std::thread f{[this, format] {
    // We must acquire the controller lock so that the main render loop
    // won't try to re-render as we modify the list of drives. This also
    // allows us to explicitly update the display at the end, so there isn't
    // a pointless wait after the drive is completed.
    auto &controller = Controller::instance();
    std::lock_guard<std::mutex> lock{controller.lock()};

    this->newdrive().piso().add_drive(this->newdrive().selected_size(), format);
    m_formatting = false;

    this->newdrive().finished_format();
    piso_log("FormatItem::add_new_formatted_drive() finished");

    Display::instance().update(this->newdrive().piso().render().first);
  }};
  f.detach();
}

FormatItem::FormatItem(NewDriveItem &new_drive)
    : m_new_drive{new_drive}, m_formatting{false},
      m_format_items{
          SimpleGUIItem{
              "Windows (NTFS)",
              [this] { add_new_formatted_drive(DriveFormat::WINDOWS); }},
          SimpleGUIItem{
              "Linux (EXT3)",
              [this] { add_new_formatted_drive(DriveFormat::LINUX); }},
          SimpleGUIItem{"Mac (EXFAT)",
                        [this] { add_new_formatted_drive(DriveFormat::MAC); }},
          SimpleGUIItem{
              "Universal (FAT32)",
              [this] { add_new_formatted_drive(DriveFormat::UNIVERSAL); }},
      } {
  update_list_items();
}

void FormatItem::update_list_items() {
  piso_log("FormatItem: Updating menu items");
  m_list_items.clear();
  for (auto &item : m_format_items) {
    m_list_items.push_back(&item);
  }

  for (const auto &item : m_list_items) {
    item->on_lose_focus();
  }
  m_selection = m_list_items.begin();
  if (has_selection()) {
    (*m_selection)->on_focus();
  }
}

std::pair<Bitmap, GUIRenderable::RenderMode> FormatItem::render() const {
  piso_log("FormatItem::render()");
  Bitmap bitmap{0, 0};
  for (const auto &item : m_format_items) {
    auto item_bitmap = item.render().first;
    auto old_height = bitmap.height();
    bitmap.expand_height(item_bitmap.height());

    if (item_bitmap.width() > bitmap.width()) {
      bitmap.expand_width(item_bitmap.width() - bitmap.width());
    }
    bitmap.blit(item_bitmap, {0, old_height});
  }
  return {bitmap, GUIRenderable::RenderMode::FULLSCREEN};
}

bool NewDriveItem::on_select() {
  piso_log("NewDriveItem::on_select()");
  switch (m_state) {
  case State::NORMAL: {
    m_state = State::SELECTING_SIZE;
    break;
  }
  case State::SELECTING_SIZE: {
    m_state = State::SELECTING_FORMAT;
    m_format_item.on_focus();
  } break;
  case State::SELECTING_FORMAT: {
    piso_log("NewDriveItem::on_select() selecting format item");
    m_format_item.on_select();

    piso_log("NewDriveItem::on_select() format item lose focus");
    m_format_item.on_lose_focus();
    m_state = State::WAITING;
  } break;
  case State::WAITING:
    break;
  }
  return true;
}

bool NewDriveItem::on_next() {
  piso_log("NewDriveItem::on_next()");
  if (m_state == State::SELECTING_SIZE) {
    m_current_percent -= 10;
    if (m_current_percent < 0) {
      m_current_percent = 0;
    }
    return true;
  } else if (m_state == State::SELECTING_FORMAT) {
    return m_format_item.on_next();
  }
  return false;
}

bool NewDriveItem::on_prev() {
  piso_log("NewDriveItem::on_prev()");
  if (m_state == State::SELECTING_SIZE) {
    m_current_percent += 10;
    return true;
  } else if (m_state == State::SELECTING_FORMAT) {
    return m_format_item.on_prev();
  }
  return false;
}

bool NewDriveItem::on_focus() {
  piso_log("NewDriveItem::on_focus()");
  if (m_state == State::SELECTING_FORMAT) {
    return m_format_item.on_focus();
  }
  return GUIEventHandler::on_focus();
}

bool NewDriveItem::on_lose_focus() {
  piso_log("NewDriveItem::on_lose_focus()");
  if (m_state == State::SELECTING_FORMAT) {
    return m_format_item.on_lose_focus();
  }
  return GUIEventHandler::on_lose_focus();
}

void NewDriveItem::finished_format() {
  // The drive has been created and formatted
  if (m_state == State::WAITING && !m_format_item.formatting()) {
    m_state = State::NORMAL;
  }
}

std::pair<Bitmap, GUIRenderable::RenderMode> NewDriveItem::render() const {
  piso_log("NewDriveItem::render()");

  switch (m_state) {
  case State::NORMAL: {
    auto text = render_text("Add new drive");

    Bitmap indented(text.width() + MENU_INDENT, text.height());
    indented.blit(text, {MENU_INDENT, 0});
    if (m_focused) {
      indented.blit(selector, {0, 0});
      return {indented, GUIRenderable::RenderMode::NORMAL};
    } else {
      return {indented, GUIRenderable::RenderMode::NORMAL};
    }
  }
  case State::SELECTING_SIZE: {
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
  case State::SELECTING_FORMAT: {
    return m_format_item.render();
  }
  case State::WAITING: {
    Bitmap disp(Display::width, Display::height);
    auto text = render_text("Formatting new disk");
    disp.blit(text, {0, 0});
    return {disp, GUIRenderable::RenderMode::FULLSCREEN};
  }
  }
}

uint64_t NewDriveItem::selected_size() const {
  unsigned long long new_size = m_current_percent / 100.0 * m_piso.size();

  // sizes must be a multiple of 512
  return ((new_size + 512 - 1) / 512) * 512;
}
