#ifndef GUIEVENTHANDLER_HPP
#define GUIEVENTHANDLER_HPP

#include <functional>
#include <utility>
#include <vector>

class GUIEventHandler {
public:
  virtual ~GUIEventHandler() {}

  // The available events from the GUI. Returning 'true' indicates
  // that the handler has handled the event. Otherwise, the parent
  // should attempt to handle it.
  virtual bool on_select() = 0;
  virtual bool on_next() = 0;
  virtual bool on_prev() = 0;

  virtual bool on_focus() {
    m_focused = true;
    return true;
  }
  virtual bool on_lose_focus() {
    m_focused = false;
    return true;
  }

protected:
  bool m_focused;
};

class Bitmap;
class GUIRenderable {
public:
  enum class RenderMode {
    NORMAL,
    FULLSCREEN,
  };

  virtual ~GUIRenderable() {}
  virtual std::pair<Bitmap, RenderMode> render() const = 0;
};

class GUIItem : public GUIEventHandler, public GUIRenderable {
public:
  virtual ~GUIItem() {}
};

class GUIListItem : public GUIItem {
protected:
  std::vector<GUIItem *> m_list_items;
  std::vector<GUIItem *>::iterator m_selection = m_list_items.end();

  virtual void update_list_items() = 0;
  virtual bool has_selection() const;

public:
  virtual ~GUIListItem() {}

  virtual bool on_focus() override;
  virtual bool on_lose_focus() override;
  virtual bool on_select() override;
  virtual bool on_next() override;
  virtual bool on_prev() override;
};

class SimpleGUIItem : public GUIItem {
protected:
  std::string m_text;
  std::function<void()> m_callback;

public:
  template <typename Callback>
  SimpleGUIItem(const std::string &text, Callback callback)
      : m_text{text}, m_callback{callback} {}

  virtual std::pair<Bitmap, GUIRenderable::RenderMode> render() const override;
  virtual bool on_select() override;
  virtual bool on_next() override { return false; }
  virtual bool on_prev() override { return false; }
};

#endif
