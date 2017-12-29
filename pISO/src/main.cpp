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
  controller.on_move = [&](Controller::Direction dir) {
    if (dir == Controller::Direction::UP) {
      piso.on_prev();
    } else {
      piso.on_next();
    }
    Display::instance().update(piso.render());
  };
  controller.on_select = [&]() {
    piso.on_select();
    Display::instance().update(piso.render());
  };
  controller.start();

  while (true) {
    Display::instance().update(piso.render());
    std::this_thread::sleep_for(std::chrono::seconds(10));
  }
}
