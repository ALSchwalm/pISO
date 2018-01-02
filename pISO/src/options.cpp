#include "options.hpp"
#include "config.hpp"
#include "font.hpp"

Bitmap add_selector(Bitmap bitmap, bool should_add) {
  Bitmap indented(bitmap.width() + MENU_INDENT, bitmap.height());
  indented.blit(bitmap, {MENU_INDENT, 0});
  if (should_add) {
    indented.blit(selector, {0, 0});
  }
  return indented;
}

std::pair<Bitmap, GUIRenderable::RenderMode> RemoveDriveItem::render() const {
  return {add_selector(render_text("Remove Drive"), m_focused),
          GUIRenderable::RenderMode::NORMAL};
}

std::pair<Bitmap, GUIRenderable::RenderMode> SnapshotItem::render() const {
  return {add_selector(render_text("Take Snapshot"), m_focused),
          GUIRenderable::RenderMode::NORMAL};
}

bool OptionsItem::OptionsHeading::on_select() {
  m_options.toggle_open();
  return true;
}

std::pair<Bitmap, GUIRenderable::RenderMode>
OptionsItem::OptionsHeading::render() const {
  piso_log("OptionsHeading: render");
  return {add_selector(render_text("Options"), m_focused),
          GUIRenderable::RenderMode::NORMAL};
}

OptionsItem::OptionsItem() : m_open{false}, m_heading{*this} {
  update_list_items();
}

std::pair<Bitmap, GUIRenderable::RenderMode> OptionsItem::render() const {
  piso_log("OptionsItem: render");
  auto bitmap = m_heading.render();
  if (!m_open) {
    return bitmap;
  }

  for (auto iter = ++m_list_items.begin(); iter != m_list_items.end(); ++iter) {
    auto item_bitmap = (*iter)->render().first;
    auto old_height = bitmap.first.height();
    bitmap.first.expand_height(item_bitmap.height());
    if (item_bitmap.width() + OPTION_INDENT > bitmap.first.width()) {
      bitmap.first.expand_width(item_bitmap.width() - bitmap.first.width() +
                                OPTION_INDENT);
    }
    bitmap.first.blit(item_bitmap, {OPTION_INDENT, old_height});
  }
  return bitmap;
}

bool OptionsItem::toggle_open() {
  m_open = !m_open;
  update_list_items();
  return m_open;
}

void OptionsItem::update_list_items() {
  piso_log("OptionsItem: Updating menu items");
  m_list_items.clear();

  m_list_items.push_back(&m_heading);
  if (m_open) {
    m_list_items.push_back(&m_remove_drive);
    m_list_items.push_back(&m_snapshot);
  }
  for (const auto &item : m_list_items) {
    item->on_lose_focus();
  }
  m_selection = m_list_items.begin();
  if (has_selection()) {
    (*m_selection)->on_focus();
  }
}
