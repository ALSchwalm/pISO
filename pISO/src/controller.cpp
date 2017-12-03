#include "controller.hpp"
#include "error.hpp"

Controller::Controller() {
  pinMode(RoAPin, INPUT);
  pinMode(RoBPin, INPUT);

  pullUpDnControl(RoAPin, PUD_UP);
  pullUpDnControl(RoBPin, PUD_UP);
}

unsigned char Controller::read_a() const { return digitalRead(RoAPin); }

unsigned char Controller::read_b() const { return digitalRead(RoBPin); }

int Controller::cw_error_check() {
  int i;
  if (((i = millis()) - m_error_time) < 300) {
    if (m_current_rotation == Rotation::CCW)
      return 0;
  }
  m_error_time = i;
  m_current_rotation = Rotation::CW;
  return 1;
}

// returns true if no error
int Controller::ccw_error_check() {
  int i;
  if (((i = millis()) - m_error_time) < 300) {
    if (m_current_rotation == Rotation::CW)
      return 0;
  }
  m_error_time = i;
  m_current_rotation = Rotation::CCW;
  return 1;
}

void Controller::rotary_deal() {
  // get current state
  m_pstate = rot_state();

  // wait until an update
  while (m_pstate == (m_nstate = rot_state())) {
    delayMicroseconds(20000);
  }

  // update
  if ((m_pstate == 0) && (m_nstate == 2)) {
    if (cw_error_check()) {
      on_rotate(Rotation::CW);
    }
  } else if ((m_pstate == 0) && (m_nstate == 1)) {
    if (ccw_error_check()) {
      on_rotate(Rotation::CCW);
    }
  } else if ((m_pstate == 3) && (m_nstate == 2)) {
    if (ccw_error_check()) {
      on_rotate(Rotation::CCW);
    }
  } else if ((m_pstate == 3) && (m_nstate == 1)) {
    if (cw_error_check()) {
      on_rotate(Rotation::CW);
    }
  }

  // debouncing
  delayMicroseconds(15000);
  while (read_a() != read_b())
    ;
}

void Controller::rotation_worker() {
  while (true) {
    rotary_deal();
  }
}

void Controller::start() {
  m_rotation_thread = std::thread{&Controller::rotation_worker, this};
}
