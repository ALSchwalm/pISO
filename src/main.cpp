#include "bitmap.hpp"
#include "font.hpp"
#include <lvm2app.h>

// Setup script sets up:
//   sudo vgcreate VolGroup00 /dev/sdb1
//   sudo lvcreate -l 100%FREE -T VolGroup00/mythinpool
//  So we can basically just do:
//   sudo lvcreate -V 100G -T VolGroup00/mythinpool -n thinvolume

int main() {
  // auto bitmap = render_text("The swift brown fox jumps over the lazy dog!");
  // auto bitmap2 = render_text("Foo");
  // bitmap.blit(bitmap2, {10, 5}, true);
  // gen_pbm(bitmap, "out.pbm");

  lvm_t handle = lvm_init(NULL);
  if (handle == NULL) {
    std::cout << "lvm_init(): " << lvm_errmsg(handle) << std::endl;
    return EXIT_FAILURE;
  }

  if (lvm_scan(handle) == -1) {
    std::cout << "lvm_scan(): " << lvm_errmsg(handle) << std::endl;
    return EXIT_FAILURE;
  }

  vg_t vg;
  struct dm_list *vgnames;
  struct lvm_str_list *strl;

  std::cout << "Current contents:" << std::endl;

  vgnames = lvm_list_vg_names(handle);
  dm_list_iterate_items(strl, vgnames) {
    auto vgname = strl->str;

    std::cout << vgname << std::endl;

    vg = lvm_vg_open(handle, vgname, "r", 0);
    if (vg == NULL) {
      std::cout << "lvm_vg_open(): " << lvm_errmsg(handle) << std::endl;
      return EXIT_FAILURE;
    }

    auto logical_volumes = lvm_vg_list_lvs(vg);
    struct lvm_lv_list *lv_list;
    dm_list_iterate_items(lv_list, logical_volumes) {
      auto lv = lv_list->lv;
      std::cout << "  " << lvm_lv_get_name(lv) << std::endl;
    }

    lvm_vg_close(vg);
  }

  vg = lvm_vg_open(handle, "VolGroup00", "w", 0);

  if (vg == NULL) {
    std::cout << "lvm_vg_open(): " << lvm_errmsg(handle) << std::endl;
    return EXIT_FAILURE;
  }

  auto thin_volume_params =
      lvm_lv_params_create_thin(vg, "mythinpool", "testvolume", 1024 * 100);

  if (thin_volume_params == NULL) {
    std::cout << "lvm_lv_params_create_thin(): " << lvm_errmsg(handle)
              << std::endl;
    return EXIT_FAILURE;
  }

  auto lv = lvm_lv_create(thin_volume_params);
  if (lv == NULL) {
    std::cout << "lvm_lv_create(): " << lvm_errmsg(handle) << std::endl;
    return EXIT_FAILURE;
  }

  lvm_vg_close(vg);
  lvm_quit(handle);
}
