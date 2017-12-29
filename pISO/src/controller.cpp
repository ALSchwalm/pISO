#include "controller.hpp"
#include "error.hpp"

Controller::Controller()
    : m_down{DOWN_PIN, GPIO::GPIO_PULL::UP}, m_up{UP_PIN, GPIO::GPIO_PULL::UP},
      m_select{SELECT_PIN, GPIO::GPIO_PULL::UP} {}

void Controller::start() {
  m_down.f_pushed = [this]() { this->on_move(Direction::DOWN); };
  m_down.start();

  m_up.f_pushed = [this]() { this->on_move(Direction::UP); };
  m_up.start();

  m_select.f_pushed = on_select;
  m_select.start();
}
