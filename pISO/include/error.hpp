#ifndef ERROR_HPP
#define ERROR_HPP

#include <iostream>
#include <stdexcept>

[[noreturn]] inline void piso_error(const std::string &s) {
  throw std::runtime_error(s);
}

template <typename... Args>
inline void piso_error(const std::string &s, const std::string &arg,
                       Args... args) {
  piso_error(s + arg, args...);
}

template <typename T> inline void piso_log(const T &s) {
  std::cout << s << std::endl;
}

template <typename T, typename U, typename... Args>
inline void piso_log(const T &s, const U &arg, Args... args) {
  std::cout << s;
  piso_log(arg, args...);
}

#endif
