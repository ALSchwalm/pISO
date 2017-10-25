
#include "virtualdrive.hpp"
#include "error.hpp"
#include "font.hpp"
#include <errno.h>
#include <sys/stat.h>
#include <sys/types.h>

bool VirtualDriveHeading::on_select() {
  if (m_vdrive.is_mounted()) {
    m_vdrive.unmount();
  } else {
    m_vdrive.mount();
  }
  return true;
}

Bitmap VirtualDriveHeading::render() const {
  return render_text(m_vdrive.name());
}

VirtualDrive::VirtualDrive(lv_t volume)
    : m_volume{volume}, m_heading{*this}, m_selection{m_list_items.end()} {}

VirtualDrive::VirtualDrive(VirtualDrive &&other)
    : m_volume{other.m_volume}, m_isos{other.m_isos},
      m_mounted{other.m_mounted}, m_heading{*this} {
  update_list_items();
}

VirtualDrive &VirtualDrive::operator=(VirtualDrive &&other) {
  m_volume = std::move(other.m_volume);
  m_isos = std::move(other.m_isos);
  m_mounted = std::move(other.m_mounted);

  update_list_items();
}

bool VirtualDrive::mount() {
  auto base_mount = getenv("MULTITOOL_BASE_MOUNT");
  if (base_mount == NULL) {
    multitool_error("getenv: cannot find 'MULTITOOL_BASE_MOUNT'");
  }
  auto path = std::string(base_mount) + "/" + this->name();
  if (mkdir(path.c_str(), 0777) == -1 && errno != EEXIST) {
    multitool_error("Cannot create path: ", path);
  }

  auto scripts_path = getenv("MULTITOOL_SCRIPTS_PATH");
  if (scripts_path == NULL) {
    multitool_error("getenv: cannot find 'MULTITOOL_SCRIPTS_PATH'");
  }
  auto vdrive_script = scripts_path + std::string("/vdrive.sh");
  FILE *proc = popen(
      ("sh " + vdrive_script + " mount " + this->name() + " " + path).c_str(),
      "r");
  if (proc == NULL) {
    multitool_error("popen: vdrive.sh mount failed");
  }

  char buff[1024];
  while (fgets(buff, sizeof(buff) - 1, proc) != NULL) {
    buff[strcspn(buff, "\n")] = 0;
    m_isos.emplace_back(buff);
  }
  pclose(proc);

  update_list_items();
  return true;
}

bool VirtualDrive::unmount() {
  auto path = "/mnt/" + this->name();
  if (system(("sh scripts/vdrive.sh unmount " + path).c_str()) != 0) {
    multitool_error("vdrive.sh unmount failed");
  }
  return true;
}

bool VirtualDrive::has_selection() const {
  return m_selection != m_list_items.end();
}

void VirtualDrive::update_list_items() {
  m_list_items.clear();
  m_list_items.push_back(&m_heading);
  for (auto &iso : m_isos) {
    m_list_items.push_back(&iso);
  }
  m_selection = m_list_items.begin();
}

bool VirtualDrive::on_select() {
  if (has_selection()) {
    return (*m_selection)->on_select();
  } else {
    return false;
  }
}

bool VirtualDrive::on_next() {
  if (has_selection()) {
    if (!(*m_selection)->on_next()) {
      m_selection++;
    }
    return true;
  } else {
    return false;
  }
}

bool VirtualDrive::on_prev() {
  if (has_selection()) {
    if (!(*m_selection)->on_prev()) {
      if (m_selection != m_list_items.begin()) {
        m_selection--;
      } else {
        return false;
      }
    }
    return true;
  } else {
    return false;
  }
}

Bitmap VirtualDrive::render() const {
  auto bitmap = m_heading.render();
  for (const auto &iso : m_isos) {
    auto iso_bitmap = iso.render();
    auto old_height = bitmap.height();
    bitmap.expand_height(iso_bitmap.height());
    if (iso_bitmap.width() > bitmap.width()) {
      bitmap.expand_width(iso_bitmap.width() - bitmap.width());
    }
    bitmap.blit(iso_bitmap, {0, old_height});
  }
  return bitmap;
}
