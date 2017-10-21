#ifndef BITMAP_HPP
#define BITMAP_HPP

#include <iostream>
#include <vector>

class Bitmap {
private:
  std::vector<std::vector<char>> m_map;

public:
  using iterator = decltype(m_map)::iterator;
  using const_iterator = decltype(m_map)::const_iterator;
  using reference = decltype(m_map)::reference;
  using const_reference = decltype(m_map)::const_reference;
  using position_t = std::pair<uint16_t, uint16_t>;

  iterator begin() { return m_map.begin(); }
  const_iterator begin() const { return m_map.begin(); }

  iterator end() { return m_map.end(); }
  const_iterator end() const { return m_map.end(); }

  reference at(std::size_t pos) { return m_map.at(pos); }
  const_reference at(std::size_t pos) const { return m_map.at(pos); }

  reference operator[](std::size_t pos) { return this->at(pos); }
  const_reference operator[](std::size_t pos) const { return this->at(pos); }

  void resize(std::size_t count) { m_map.resize(count); }

  std::size_t height() const { return m_map.size(); }
  std::size_t width() const {
    if (m_map.size() == 0) {
      return 0;
    }
    return m_map[0].size();
  }

  void blit(const Bitmap &, position_t position, bool transparent = false);

  Bitmap() : m_map{} {}
  Bitmap(const std::initializer_list<std::vector<char>> &map) : m_map{map} {}
  Bitmap(std::size_t width, std::size_t height)
      : m_map(height, std::vector<char>(width, 0)) {}
};

std::ostream &operator<<(std::ostream &, const Bitmap &);

#endif
