#include <cppgpio.hpp>
#include <errno.h>
#include <string.h>
#include <wiringPi.h>

#include "bitmap.hpp"
#include "controller.hpp"
#include "display.hpp"
#include "font.hpp"
#include "lvmwrapper.hpp"
#include "piso.hpp"

int main() {
  if (wiringPiSetupGpio() == -1) {
    piso_error("Error while setting up GPIO: ", strerror(errno));
  }

  auto &piso = pISO::instance();
  Display::instance().update(piso.render());

  auto &controller = Controller::instance();
  controller.on_rotate = [&](Controller::Rotation rot) {
    if (rot == Controller::Rotation::CW) {
      piso.on_prev();
    } else {
      piso.on_next();
    }
    Display::instance().update(piso.render());
  };
  controller.start();

  GPIO::PushButton button(22, GPIO::GPIO_PULL::UP);
  button.f_pushed = [&]() {
    piso.on_select();
    Display::instance().update(piso.render());
  };
  button.start();

  while (true) {
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
  }
}
