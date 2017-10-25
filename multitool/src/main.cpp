#include <cppgpio.hpp>
#include <lvm2app.h>

#include "bitmap.hpp"
#include "font.hpp"
#include "multitool.hpp"

// Setup script sets up:
//   sudo vgcreate VolGroup00 /dev/sdb1
//   sudo lvcreate -l 100%FREE -T VolGroup00/thinpool
//  So we can basically just do:
//   sudo lvcreate -V 100G -T VolGroup00/thinpool -n volume0

int main() {
  // auto bitmap = render_text("The swift brown fox jumps over the lazy dog!");
  // auto bitmap2 = render_text("Foo");
  // bitmap.blit(bitmap2, {10, 5}, true);
  // gen_pbm(bitmap, "out.pbm");

  auto &multi = Multitool::instance();
  multi.on_select();

  GPIO::RotaryDial dial(17, 27, GPIO::GPIO_PULL::UP);
  dial.f_dialed = [&](bool up, long value) {
    std::cout << up << " " << value << std::endl;
    if (up) {
      multi.on_next();
    } else {
      multi.on_prev();
    }
  };
  dial.start();

  std::this_thread::sleep_for(std::chrono::hours(1));
}
