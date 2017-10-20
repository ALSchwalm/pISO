#ifndef ERROR_HPP
#define ERROR_HPP

#include <stdexcept>

[[noreturn]] inline void multitool_error(const std::string &s) {
  throw std::runtime_error(s);
}

template <typename... Args>
inline void multitool_error(const std::string &s, const std::string &arg,
                            Args... args) {
  multitool_error(s + arg, args...);
}

#endif
