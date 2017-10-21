
#include "error.hpp"
#include "multitool.hpp"
#include <errno.h>
#include <sys/stat.h>
#include <sys/types.h>

bool VirtualDrive::mount() {
  auto path = "/mnt/" + this->name();
  if (mkdir(path.c_str(), 0777) == -1 && errno != EEXIST) {
    multitool_error("Cannot create path: ", path);
  }

  // TODO: handle partitions we can't mount
  if (system(("sh scripts/vdrive.sh mount " + this->name() + " " + path)
                 .c_str()) != 0) {
    multitool_error("vdrive.sh mount failed");
  }
  return true;
}

bool VirtualDrive::unmount() {
  auto path = "/mnt/" + this->name();
  if (system(("sh scripts/vdrive.sh unmount " + path).c_str()) != 0) {
    multitool_error("vdrive.sh unmount failed");
  }
  return true;
}
