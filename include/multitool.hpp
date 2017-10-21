#ifndef MULTITOOL_HPP
#define MULTITOOL_HPP

#include "error.hpp"
#include <lvm2app.h>
#include <vector>

class VirtualDrive {
private:
  lv_t m_volume;

public:
  VirtualDrive(lv_t volume) : m_volume{volume} {}

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
};

inline bool operator==(const VirtualDrive &left, const VirtualDrive &right) {
  return left.volume() == right.volume();
}

class Multitool {
private:
  lvm_t m_lvm;
  vg_t m_volgroup;
  lv_t m_thinpool;
  std::vector<VirtualDrive> m_drives;

public:
  Multitool();
  ~Multitool();
  Multitool(const Multitool &other) = delete;

  std::vector<VirtualDrive> &drives() { return m_drives; }
  const std::vector<VirtualDrive> &drives() const { return m_drives; }
  const VirtualDrive &add_drive(uint64_t size);
  void remove_drive(const VirtualDrive &drive);
  void rescan_drives();

  float percent_used() const;
};

#endif
