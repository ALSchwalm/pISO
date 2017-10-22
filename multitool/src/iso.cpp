
#include "iso.hpp"
#include "font.hpp"
#include <iostream>
#include <libgen.h>

bool ISO::mount() {
  std::cout << "Mounted: " << m_path << std::endl;
  return true;
}

bool ISO::unmount() { return true; }

bool ISO::on_select() {
  if (m_mounted) {
    unmount();
  } else {
    mount();
  }
  return true;
}

Bitmap ISO::render() const {
  auto buff = new char[m_path.size() + 1];
  m_path.copy(buff, m_path.size() + 1);
  auto bitmap = render_text(basename(buff));
  return bitmap;
}
