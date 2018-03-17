#ifndef NEWDRIVE_HPP
#define NEWDRIVE_HPP

#include "guiitem.hpp"

class pISO;
class NewDriveItem;
enum class DriveFormat;

class FormatItem : public GUIListItem {
  virtual void update_list_items() override;
  NewDriveItem &m_new_drive;
  bool m_formatting;
  std::array<SimpleGUIItem, 4> m_format_items;

  void add_new_formatted_drive(const DriveFormat &format);

public:
  FormatItem(NewDriveItem &);
  virtual ~FormatItem() {}

  virtual std::pair<Bitmap, GUIRenderable::RenderMode> render() const override;

  bool formatting() const { return m_formatting; }
  NewDriveItem &newdrive() { return m_new_drive; }
};

class NewDriveItem : public GUIItem {
  enum class State {
    NORMAL,           // User hasn't selected new drive
    SELECTING_SIZE,   // User has selected the new drive option, is now picking
                      // size
    SELECTING_FORMAT, // User has selected a size, is now picking format
    WAITING // User has picked a size and is waiting for the format to finish
  };

  State m_state;
  pISO &m_piso;
  FormatItem m_format_item;
  int m_current_percent;

public:
  NewDriveItem(pISO &piso)
      : m_state{State::NORMAL}, m_piso{piso}, m_format_item{*this},
        m_current_percent{50} {}
  virtual ~NewDriveItem() {}

  virtual bool on_select() override;
  virtual bool on_next() override;
  virtual bool on_prev() override;
  virtual bool on_focus() override;
  virtual bool on_lose_focus() override;

  virtual std::pair<Bitmap, GUIRenderable::RenderMode> render() const override;

  pISO &piso() { return m_piso; }
  uint64_t selected_size() const;

  // This is a hack for the FormatItem to notify the NewDriveItem that the
  // format has been completed so it can update its state.
  void finished_format();
};

#endif
