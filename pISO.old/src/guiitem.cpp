#include "guiitem.hpp"
#include "bitmap.hpp"
#include "error.hpp"
#include "font.hpp"

bool GUIListItem::has_selection() const {
  return m_selection != m_list_items.end();
}

bool GUIListItem::on_focus() {
  GUIEventHandler::on_focus();
  if (has_selection()) {
    return (*m_selection)->on_focus();
  }
  return false;
}

bool GUIListItem::on_lose_focus() {
  GUIEventHandler::on_lose_focus();
  if (has_selection()) {
    return (*m_selection)->on_lose_focus();
  }
  return false;
}

bool GUIListItem::on_select() {
  if (has_selection()) {
    return (*m_selection)->on_select();
  } else {
    return false;
  }
}

bool GUIListItem::on_next() {
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

bool GUIListItem::on_prev() {
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

std::pair<Bitmap, GUIRenderable::RenderMode> SimpleGUIItem::render() const {
  piso_log("SimpleGUIItem::on_render");
  return {add_selector(render_text(m_text), m_focused),
          GUIRenderable::RenderMode::NORMAL};
}

bool SimpleGUIItem::on_select() {
  piso_log("SimpleGUIItem::on_select");
  m_callback();
  return true;
}
