#ifndef VIRTUALDRIVE_HPP
#define VIRTUALDRIVE_HPP

#include "error.hpp"
#include "guiitem.hpp"
#include <iostream> //TODO: remove this
#include <lvm2app.h>
#include <vector>

class ISO : public GUIItem {
  std::string m_path;
  bool m_mounted = false;

public:
  ISO(const std::string &path) : m_path{path} {}
  virtual ~ISO() {}

  bool mount() {
    std::cout << "Mounted: " << m_path << std::endl;
    return true;
  }
  bool unmount() { return true; }

  virtual bool on_select() override {
    if (m_mounted) {
      unmount();
    } else {
      mount();
    }
    return true;
  }
  virtual bool on_next() override { return false; }
  virtual bool on_prev() override { return false; }
};

class VirtualDrive : public GUIItem {
private:
  lv_t m_volume;
  std::vector<ISO> m_isos;

  std::vector<GUIItem *> m_list_items;
  std::vector<GUIItem *>::iterator m_selection;

  void update_list_items();
  bool has_selection() const;

public:
  VirtualDrive(lv_t volume);
  virtual ~VirtualDrive() {}

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

  virtual bool on_select() override;
  virtual bool on_next() override;
  virtual bool on_prev() override;
};

inline bool operator==(const VirtualDrive &left, const VirtualDrive &right) {
  return left.volume() == right.volume();
}

#endif
