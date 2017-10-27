#ifndef LVMWRAPPER_HPP
#define LVMWRAPPER_HPP

#include <json/json.h>
#include <sstream>
#include <string>

namespace detail {
inline void join_args_impl(std::ostringstream &) { return; }

template <typename T, typename... Args>
void join_args_impl(std::ostringstream &os, T arg, Args... args) {
  os << arg;
  join_args_impl(os, std::forward<Args>(args)...);
}

template <typename... Args> std::string join_args(Args &&... args) {
  std::ostringstream os;
  join_args_impl(os, std::forward<Args>(args)...);
  return os.str();
}

std::string run_command_impl(const std::string &cmd);

std::string lvm_run_impl(const std::string &);
Json::Value lvm_run_json_impl(const std::string &);
} // namespace detail

template <typename... Args> inline std::string run_command(Args &&... args) {
  return detail::run_command_impl(
      detail::join_args(std::forward<Args>(args)...));
}

template <typename... Args> inline std::string lvm_run(Args &&... args) {
  return detail::lvm_run_impl(detail::join_args(std::forward<Args>(args)...));
}

template <typename... Args> inline Json::Value lvm_run_json(Args &&... args) {
  return detail::lvm_run_json_impl(
      detail::join_args(std::forward<Args>(args)...));
}

Json::Value lvm_lvs_report(std::string options = "", std::string volname = "");

#endif
