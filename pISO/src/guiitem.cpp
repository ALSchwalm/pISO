#include "guiitem.hpp"
#include "error.hpp"

bool GUIListItem::has_selection() const {
  return m_selection != m_list_items.end();
}

bool GUIListItem::on_focus() {
  GUIEventHandler::on_focus();
  if (has_selection()) {
    (*m_selection)->on_focus();
  }
}

bool GUIListItem::on_lose_focus() {
  GUIEventHandler::on_lose_focus();
  if (has_selection()) {
    (*m_selection)->on_lose_focus();
  }
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
