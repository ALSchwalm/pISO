#ifndef PISO_HPP
#define PISO_HPP

#include "error.hpp"
#include "guiitem.hpp"
#include "virtualdrive.hpp"

#include <usbg/usbg.h>
#include <vector>

class pISO;
class NewDriveItem : public GUIItem {
  pISO &m_piso;
  bool m_selecting_size;
  int m_current_percent;

public:
  NewDriveItem(pISO &piso)
      : m_piso{piso}, m_selecting_size{false}, m_current_percent{100} {}
  virtual ~NewDriveItem() {}

  virtual bool on_select() override;
  virtual bool on_next() override;
  virtual bool on_prev() override;

  virtual std::pair<Bitmap, GUIRenderable::RenderMode> render() const override;
};

class pISO : public GUIListItem {
private:
  static const int VENDOR_ID = 0x1d6b;  // Linux Foundation// Linux Foundation
  static const int PRODUCT_ID = 0x0104; // Multifunction Composite Gadget
  static const int SIDEBAR_SPACE = 2;
  static const int MENU_LEFT_SPACE = 3;

  std::vector<VirtualDrive> m_drives;
  NewDriveItem m_newdrive;
  usbg_gadget *m_gadget;
  usbg_config *m_usb_config;

  void update_list_items() override;

  pISO();
  pISO(const pISO &) = delete;
  pISO &operator=(const pISO &) = delete;

  void rebuild_drives_from_volumes();
  void init_usbgx();

public:
  virtual ~pISO(){};

  static pISO &instance() {
    static pISO piso;
    return piso;
  }

  std::vector<VirtualDrive> &drives() { return m_drives; }
  const std::vector<VirtualDrive> &drives() const { return m_drives; }
  const VirtualDrive &add_drive(uint64_t size);
  void remove_drive(const VirtualDrive &drive);

  float percent_used() const;
  unsigned long long size() const;

  virtual bool on_select() override;
  virtual bool on_next() override;
  virtual bool on_prev() override;

  virtual std::pair<Bitmap, GUIRenderable::RenderMode> render() const override;
};

#endif
