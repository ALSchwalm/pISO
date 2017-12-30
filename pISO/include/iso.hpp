#ifndef ISO_HPP
#define ISO_HPP

#include "guiitem.hpp"
#include <string>

class ISO : public GUIItem {
  std::string m_path;
  bool m_mounted = false;

public:
  ISO(const std::string &path) : m_path{path} {}
  virtual ~ISO() {}

  bool mount();
  bool unmount();
  bool is_mounted() const { return m_mounted; }

  virtual bool on_select() override;
  virtual bool on_next() override { return false; }
  virtual bool on_prev() override { return false; }

  virtual std::pair<Bitmap, GUIRenderable::RenderMode> render() const override;
};

#endif
