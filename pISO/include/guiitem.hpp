#ifndef GUIEVENTHANDLER_HPP
#define GUIEVENTHANDLER_HPP

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
  enum RenderMode {
    NORMAL,
    FULLSCREEN,
  };

  virtual ~GUIRenderable() {}
  virtual Bitmap render() const = 0;
};

class GUIItem : public GUIEventHandler, public GUIRenderable {
public:
  virtual ~GUIItem() {}
};

#endif
