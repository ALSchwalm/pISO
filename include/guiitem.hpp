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
};

class GUIRenderable {
public:
  enum RenderMode {
    NORMAL,
    FULLSCREEN,
  };

  virtual ~GUIRenderable() {}
};

class GUIItem : public GUIEventHandler, public GUIRenderable {
public:
  virtual ~GUIItem() {}
};

#endif
