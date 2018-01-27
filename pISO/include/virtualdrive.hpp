#ifndef VIRTUALDRIVE_HPP
#define VIRTUALDRIVE_HPP

#include "error.hpp"
#include "guiitem.hpp"
#include "iso.hpp"
#include "lvmwrapper.hpp"
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

  virtual std::pair<Bitmap, GUIRenderable::RenderMode> render() const override;
};

enum class DriveFormat { WINDOWS, LINUX, MAC, UNIVERSAL };

class VirtualDrive : public GUIListItem {
public:
  enum class MountState { UNMOUNTED, INTERNAL, EXTERNAL };

private:
  const int ISO_LABEL_INDENT = 5;
  std::string m_volume_name;
  std::string m_uuid;
  unsigned long long m_size;

  std::vector<ISO> m_isos;
  MountState m_mount_state = MountState::UNMOUNTED;

  VirtualDriveHeading m_heading;

protected:
  virtual void update_list_items() override;

public:
  VirtualDrive(const std::string &volume_name);
  VirtualDrive(VirtualDrive &&);
  VirtualDrive &operator=(VirtualDrive &&);
  virtual ~VirtualDrive() {}

  VirtualDrive(const VirtualDrive &) = delete;
  VirtualDrive &operator=(const VirtualDrive &) = delete;

  std::string name() const { return m_volume_name; }
  std::string uuid() const { return m_uuid; }
  unsigned long long size() const { return m_size; }
  float percent_used() const;

  bool mount_internal();
  bool unmount_internal();
  bool mount_external();
  bool unmount_external();
  MountState mount_state() const { return m_mount_state; }

  virtual bool on_focus() override;
  virtual bool on_lose_focus() override;
  virtual bool on_select() override;
  virtual bool on_next() override;
  virtual bool on_prev() override;

  virtual std::pair<Bitmap, GUIRenderable::RenderMode> render() const override;
};

inline bool operator==(const VirtualDrive &left, const VirtualDrive &right) {
  // TODO: this should be based on UUID
  return left.name() == right.name();
}

#endif
