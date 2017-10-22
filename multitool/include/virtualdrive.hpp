#ifndef VIRTUALDRIVE_HPP
#define VIRTUALDRIVE_HPP

#include "error.hpp"
#include "guiitem.hpp"
#include "iso.hpp"
#include <iostream> //TODO: remove this
#include <lvm2app.h>
#include <vector>

class VirtualDrive;
class VirtualDriveHeading : public GUIItem {
  VirtualDrive &m_vdrive;

public:
  VirtualDriveHeading(VirtualDrive &vdrive) : m_vdrive{vdrive} {}
  virtual ~VirtualDriveHeading() {}

  virtual bool on_select() override;
  virtual bool on_next() override { return false; }
  virtual bool on_prev() override { return false; }

  virtual Bitmap render() const override;
};

class VirtualDrive : public GUIItem {
private:
  lv_t m_volume;
  std::vector<ISO> m_isos;
  bool m_mounted = false;

  VirtualDriveHeading m_heading;
  std::vector<GUIItem *> m_list_items;
  std::vector<GUIItem *>::iterator m_selection;

  void update_list_items();
  bool has_selection() const;

public:
  VirtualDrive(lv_t volume);
  VirtualDrive(VirtualDrive &&);
  VirtualDrive &operator=(VirtualDrive &&);
  virtual ~VirtualDrive() {}

  VirtualDrive(const VirtualDrive &) = delete;
  VirtualDrive &operator=(const VirtualDrive &) = delete;

  std::string name() const { return lvm_lv_get_name(m_volume); }
  std::string uuid() const { return lvm_lv_get_uuid(m_volume); }
  uint64_t size() const { return lvm_lv_get_size(m_volume); }
  float percent_used() const {
    auto prop = lvm_lv_get_property(m_volume, "data_percent");
    if (!prop.is_valid) {
      multitool_error("data_percent is not a valid property");
    }
    return lvm_percent_to_float(prop.value.integer);
  }
  lv_t volume() const { return m_volume; }

  bool mount();
  bool unmount();
  bool is_mounted() const { return m_mounted; }

  virtual bool on_select() override;
  virtual bool on_next() override;
  virtual bool on_prev() override;

  virtual Bitmap render() const override;
};

inline bool operator==(const VirtualDrive &left, const VirtualDrive &right) {
  return left.volume() == right.volume();
}

#endif
