#ifndef CONTROLLER_HPP
#define CONTROLLER_HPP

#include <functional>
#include <thread>
#include <wiringPi.h>

class Controller {
public:
  enum class Rotation { CW = 1, CCW = 0, UNKNOWN = 2 };

private:
  static const int RoAPin = 17;
  static const int RoBPin = 27;

  unsigned char read_a() const;
  unsigned char read_b() const;
  unsigned char rot_state() const { return (read_a() << 1) | read_b(); }

  Rotation m_current_rotation = Rotation::UNKNOWN;
  char m_pstate;
  char m_nstate;
  int m_error_time = 0;
  int m_rotation = 0;
  std::thread m_rotation_thread;
  std::thread m_button_thread;

  int cw_error_check();
  int ccw_error_check();
  void rotary_deal();
  void rotation_worker();

  Controller &operator=(const Controller &) = delete;
  Controller(const Controller &) = delete;
  Controller();

public:
  std::function<void(Rotation)> on_rotate;
  std::function<void()> on_press;

  static Controller &instance() {
    static Controller c;
    return c;
  }

  void start();
};

#endif
