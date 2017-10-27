#include "bitmap.hpp"
#include "font.hpp"
#include "lvmwrapper.hpp"
#include "multitool.hpp"
#include <cppgpio.hpp>

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

  GPIO::RotaryDial dial(17, 27, GPIO::GPIO_PULL::UP);
  dial.f_dialed = [&](bool up, long value) {
    std::cout << up << " " << value << std::endl;

    if (value > 10) {
      multi.add_drive(1024 * 1000 * 30);
    }
  };
  dial.start();

  GPIO::PushButton button(22, GPIO::GPIO_PULL::UP);
  button.f_pushed = [&]() { multi.on_select(); };
  button.start();

  std::this_thread::sleep_for(std::chrono::hours(1));
}
