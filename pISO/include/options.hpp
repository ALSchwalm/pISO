#ifndef OPTIONSLIST_HPP
#define OPTIONSLIST_HPP

#include "error.hpp"
#include "guiitem.hpp"
#include "iso.hpp"
#include "lvmwrapper.hpp"
#include <vector>

class RemoveDriveItem : public GUIItem {
public:
  virtual ~RemoveDriveItem() {}

  virtual bool on_select() override { return true; }
  virtual bool on_next() override { return false; }
  virtual bool on_prev() override { return false; }

  virtual std::pair<Bitmap, GUIRenderable::RenderMode> render() const override;
};

class SnapshotItem : public GUIItem {
public:
  virtual ~SnapshotItem() {}

  virtual bool on_select() override { return true; }
  virtual bool on_next() override { return false; }
  virtual bool on_prev() override { return false; }

  virtual std::pair<Bitmap, GUIRenderable::RenderMode> render() const override;
};

class OptionsItem : public GUIListItem {
  class OptionsHeading : public GUIItem {
    OptionsItem &m_options;

  public:
    OptionsHeading(OptionsItem &options) : m_options{options} {}
    virtual ~OptionsHeading() {}

    virtual bool on_select() override;
    virtual bool on_next() override { return false; }
    virtual bool on_prev() override { return false; }

    virtual std::pair<Bitmap, GUIRenderable::RenderMode>
    render() const override;
  };
  const int OPTION_INDENT = 5;

  bool m_open;
  OptionsHeading m_heading;
  RemoveDriveItem m_remove_drive;
  SnapshotItem m_snapshot;

protected:
  virtual void update_list_items() override;

public:
  OptionsItem();
  virtual ~OptionsItem() {}

  virtual std::pair<Bitmap, GUIRenderable::RenderMode> render() const override;

  bool toggle_open();
};

#endif
