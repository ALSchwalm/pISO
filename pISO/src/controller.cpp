#include "controller.hpp"
#include "display.hpp"
#include "error.hpp"

Controller::Controller()
    : m_down{DOWN_PIN, GPIO::GPIO_PULL::UP}, m_up{UP_PIN, GPIO::GPIO_PULL::UP},
      m_select{SELECT_PIN, GPIO::GPIO_PULL::UP} {}

void Controller::start() {
  m_down.f_pushed = [this]() {
    std::lock_guard<std::mutex> lock{this->m_controller_lock};
    this->on_move((!m_invert_input) ? Direction::DOWN : Direction::UP);
  };
  m_down.start();

  m_up.f_pushed = [this]() {
    std::lock_guard<std::mutex> lock{this->m_controller_lock};
    this->on_move((!m_invert_input) ? Direction::UP : Direction::DOWN);
  };
  m_up.start();

  m_select.f_released = [this](std::chrono::nanoseconds time) {
    std::lock_guard<std::mutex> lock{this->m_controller_lock};
    if (time > m_long_press_time) {
      this->flip_input();
      Display::instance().flip_orientation();
      this->on_long_press();
    } else {
      this->on_select();
    }
  };
  m_select.start();
}
