
#include "bitmap.hpp"

std::ostream &operator<<(std::ostream &os, const Bitmap &map) {
  for (const auto &row : map) {
    for (auto bit : row) {
      if (bit) {
        os << (int)bit;
      } else {
        os << " ";
      }
    }
    os << std::endl;
  }
  return os;
}

void Bitmap::blit(const Bitmap &other, position_t position, bool transparent) {
  for (auto i = 0; i < other.height(); ++i) {
    for (auto j = 0; j < other.width(); ++j) {
      auto offset_x = position.first;
      auto offset_y = position.second;
      if (i + offset_y < this->height() && j + offset_x < this->width() &&
          i < other.height() && j < other.width()) {
        if (!transparent || other[i][j]) {
          (*this)[i + offset_y][j + offset_x] = other[i][j];
        }
      }
    }
  }
}

Bitmap Bitmap::rotate(Direction dir) const {
  Bitmap out{height(), width()};
  for (auto y = 0; y < height(); ++y) {
    for (auto x = 0; x < width(); ++x) {
      if (dir == Direction::Left) {
        out[width() - x - 1][y] = (*this)[y][x];
      } else {
        out[x][height() - y - 1] = (*this)[y][x];
      }
    }
  }
  return out;
}

void Bitmap::expand_height(std::size_t count) {
  for (std::size_t i = 0; i < count; ++i) {
    m_map.push_back(std::vector<char>(width(), 0));
  }
}

void Bitmap::expand_width(std::size_t count) {
  for (auto &row : m_map) {
    row.insert(row.end(), count, 0);
  }
}
