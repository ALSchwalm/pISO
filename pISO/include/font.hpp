#ifndef FONT_HPP
#define FONT_HPP

#include "bitmap.hpp"

using bitmap_t = Bitmap;

extern const bitmap_t font[128];
extern const bitmap_t unprintable;
extern const bitmap_t selector;
extern const bitmap_t mount_indicator;

bitmap_t render_text(const std::string &str);
bitmap_t add_selector(bitmap_t map, bool should_add);
void gen_pbm(const bitmap_t &map, const std::string &filename);

#endif
