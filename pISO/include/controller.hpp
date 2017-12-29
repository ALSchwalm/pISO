#ifndef CONTROLLER_HPP
#define CONTROLLER_HPP

#include <cppgpio.hpp>
#include <functional>

class Controller {
public:
  enum class Direction { UP = 0, DOWN = 1 };

private:
  static const int UP_PIN = 17;
  static const int DOWN_PIN = 22;
  static const int SELECT_PIN = 27;

  GPIO::PushButton m_down;
  GPIO::PushButton m_up;
  GPIO::PushButton m_select;

  Controller &operator=(const Controller &) = delete;
  Controller(const Controller &) = delete;
  Controller();

public:
  std::function<void(Direction)> on_move;
  std::function<void()> on_select;

  static Controller &instance() {
    static Controller c;
    return c;
  }

  void start();
};

#endif
