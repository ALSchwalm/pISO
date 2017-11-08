#ifndef PISO_HPP
#define PISO_HPP

#include "error.hpp"
#include "guiitem.hpp"
#include "virtualdrive.hpp"
#include <vector>

class pISO : public GUIItem {
private:
  std::vector<VirtualDrive> m_drives;
  std::vector<GUIEventHandler *> m_list_items;
  std::vector<GUIEventHandler *>::iterator m_selection;

  void update_list_items();
  bool has_selection() const;

  pISO();
  pISO(const pISO &) = delete;
  pISO &operator=(const pISO &) = delete;

  void rebuild_drives_from_volumes();

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

  virtual bool on_select() override;
  virtual bool on_next() override;
  virtual bool on_prev() override;

  virtual Bitmap render() const override;
};

#endif
