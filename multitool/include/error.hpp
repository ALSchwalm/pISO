#ifndef ERROR_HPP
#define ERROR_HPP

#include <iostream>
#include <stdexcept>

[[noreturn]] inline void multitool_error(const std::string &s) {
  throw std::runtime_error(s);
}

template <typename... Args>
inline void multitool_error(const std::string &s, const std::string &arg,
                            Args... args) {
  multitool_error(s + arg, args...);
}

template <typename T> inline void multitool_log(const T &s) {
  std::cout << s << std::endl;
}

template <typename T, typename U, typename... Args>
inline void multitool_log(const T &s, const U &arg, Args... args) {
  std::cout << s;
  multitool_log(arg, args...);
}

#endif
