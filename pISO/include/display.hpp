#ifndef DISPLAY_HPP
#define DISPLAY_HPP

#include "bitmap.hpp"

class Display {
  enum class SSD1306_COMMAND : unsigned char {
    // Constants
    I2C_ADDRESS = 0x3C, // 011110+SA0+RW - 0x3C or 0x3D
    SETCONTRAST = 0x81,
    DISPLAYALLON_RESUME = 0xA4,
    DISPLAYALLON = 0xA5,
    NORMALDISPLAY = 0xA6,
    INVERTDISPLAY = 0xA7,
    DISPLAYOFF = 0xAE,
    DISPLAYON = 0xAF,
    SETDISPLAYOFFSET = 0xD3,
    SETCOMPINS = 0xDA,
    SETVCOMDETECT = 0xDB,
    SETDISPLAYCLOCKDIV = 0xD5,
    SETPRECHARGE = 0xD9,
    SETMULTIPLEX = 0xA8,
    SETLOWCOLUMN = 0x00,
    SETHIGHCOLUMN = 0x10,
    SETSTARTLINE = 0x40,
    MEMORYMODE = 0x20,
    COLUMNADDR = 0x21,
    PAGEADDR = 0x22,
    COMSCANINC = 0xC0,
    COMSCANDEC = 0xC8,
    SEGREMAP = 0xA0,
    CHARGEPUMP = 0x8D,
    EXTERNALVCC = 0x1,
    SWITCHCAPVCC = 0x2,

    // Scrolling constants
    ACTIVATE_SCROLL = 0x2F,
    DEACTIVATE_SCROLL = 0x2E,
    SET_VERTICAL_SCROLL_AREA = 0xA3,
    RIGHT_HORIZONTAL_SCROLL = 0x26,
    LEFT_HORIZONTAL_SCROLL = 0x27,
    VERTICAL_AND_RIGHT_HORIZONTAL_SCROLL = 0x29,
    VERTICAL_AND_LEFT_HORIZONTAL_SCROLL = 0x2A,
  };

  static const int channel = 0;
  static const int speed = 8000000;
  static const int dc_pin = 25;
  static const int rst_pin = 19;
  static const int width = 128;
  static const int height = 64;

  Bitmap m_map;
  int m_spi_fd;

  void send_spi_command(unsigned char command);
  void send_spi_command(SSD1306_COMMAND command);
  void send_spi_data(std::vector<unsigned char> &data);

  Display();
  Display(const Display &) = delete;
  Display &operator=(const Display &) = delete;

public:
  static Display &instance() {
    static Display display;
    return display;
  }

  void update(const Bitmap &);
  void reset();
};

#endif
