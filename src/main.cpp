#include <cppgpio.hpp>
#include <lvm2app.h>

#include "bitmap.hpp"
#include "font.hpp"
#include "multitool.hpp"

// Setup script sets up:
//   sudo vgcreate VolGroup00 /dev/sdb1
//   sudo lvcreate -l 100%FREE -T VolGroup00/thinpool
//  So we can basically just do:
//   sudo lvcreate -V 100G -T VolGroup00/thinpool -n thinvolume

int main() {
  // auto bitmap = render_text("The swift brown fox jumps over the lazy dog!");
  // auto bitmap2 = render_text("Foo");
  // bitmap.blit(bitmap2, {10, 5}, true);
  // gen_pbm(bitmap, "out.pbm");

  Multitool multi;
  std::cout << "Whole device: " << multi.percent_used() << std::endl;
  // std::cout << "Before anything" << std::endl;

  multi.drives().at(3).mount();
  // for (const auto &drive : multi.drives()) {
  //   std::cout << drive.name() << ": " << drive.percent_used() << std::endl;
  // };

  // GPIO::RotaryDial dial(17, 27, GPIO::GPIO_PULL::UP);
  // dial.f_dialed = [&](bool up, long value) {
  //   std::cout << up << " " << value << std::endl;
  // };
  // std::cout << "Starting dial" << std::endl;
  // dial.start();
  // std::cout << "Started dial" << std::endl;
}
