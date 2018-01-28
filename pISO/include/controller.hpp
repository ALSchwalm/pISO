#ifndef CONTROLLER_HPP
#define CONTROLLER_HPP

#include <cppgpio.hpp>
#include <functional>
#include <mutex>

class Controller {
public:
  enum class Direction { UP = 0, DOWN = 1 };

private:
  static const int UP_PIN = 27;
  static const int DOWN_PIN = 22;
  static const int SELECT_PIN = 17;

  GPIO::PushButton m_down;
  GPIO::PushButton m_up;
  GPIO::PushButton m_select;
  std::mutex m_controller_lock;
  bool m_invert_input = false;
  std::chrono::seconds m_long_press_time = std::chrono::seconds(1);

  Controller &operator=(const Controller &) = delete;
  Controller(const Controller &) = delete;
  Controller();

public:
  std::function<void(Direction)> on_move;
  std::function<void()> on_select;
  std::function<void()> on_long_press;
  std::mutex &lock() { return m_controller_lock; }

  static Controller &instance() {
    static Controller c;
    return c;
  }

  void start();
  void flip_input() { m_invert_input = !m_invert_input; }
};

#endif
