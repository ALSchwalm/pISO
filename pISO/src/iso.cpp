
#include "iso.hpp"
#include "config.hpp"
#include "error.hpp"
#include "font.hpp"
#include "lvmwrapper.hpp"
#include <iostream>
#include <libgen.h>

bool ISO::mount() {
  piso_log("ISO::mount(): mounting ", m_path);

  if (m_mounted) {
    piso_error("ISO::mount(): iso is already mounted: ", m_path);
  }

  auto scripts_path = config_getenv("PISO_SCRIPTS_PATH");
  auto iso_script = scripts_path + "/iso.sh";

  run_command("sh ", iso_script, " mount ", m_path);
  m_mounted = true;
  return true;
}

bool ISO::unmount() {
  piso_log("ISO::unmount(): unmounting ", m_path);

  if (!m_mounted) {
    piso_error("ISO::mount(): iso is already unmounted: ", m_path);
  }

  auto scripts_path = config_getenv("PISO_SCRIPTS_PATH");
  auto iso_script = scripts_path + "/iso.sh";

  run_command("sh ", iso_script, " unmount ", m_path);
  m_mounted = false;
  return true;
}

bool ISO::on_select() {
  piso_log("ISO::on_select()");
  if (m_mounted) {
    unmount();
  } else {
    mount();
  }
  return true;
}

Bitmap ISO::render() const {
  piso_log("ISO::render()");
  auto buff = new char[m_path.size() + 1];
  m_path.copy(buff, m_path.size() + 1);
  auto bitmap = render_text(basename(buff));
  delete[] buff;
  return bitmap;
}
