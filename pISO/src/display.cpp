
#include "display.hpp"
#include "error.hpp"
#include <errno.h>
#include <string.h>
#include <wiringPi.h>
#include <wiringPiSPI.h>

Display::Display() : m_map{width, height} {
  m_spi_fd = wiringPiSPISetup(channel, speed);
  if (m_spi_fd == -1) {
    piso_error("Error running wiringPiSPISetup: ", strerror(errno));
  }

  pinMode(dc_pin, OUTPUT);
  pinMode(rst_pin, OUTPUT);

  reset();
  send_spi_command(SSD1306_COMMAND::DISPLAYOFF);
  send_spi_command(SSD1306_COMMAND::SETDISPLAYCLOCKDIV);
  send_spi_command(0x80); // the suggested ratio 0x80
  send_spi_command(SSD1306_COMMAND::SETMULTIPLEX);
  send_spi_command(0x3F);
  send_spi_command(SSD1306_COMMAND::SETDISPLAYOFFSET);
  send_spi_command(0x0);                                      // no offset
  send_spi_command((int)SSD1306_COMMAND::SETSTARTLINE | 0x0); // line #0
  send_spi_command(SSD1306_COMMAND::CHARGEPUMP);
  send_spi_command(0x14);
  send_spi_command(SSD1306_COMMAND::MEMORYMODE);
  send_spi_command(0x00); // 0x0 act like ks0108
  send_spi_command((int)SSD1306_COMMAND::SEGREMAP | 0x1);
  send_spi_command(SSD1306_COMMAND::COMSCANDEC);
  send_spi_command(SSD1306_COMMAND::SETCOMPINS);
  send_spi_command(0x12);
  send_spi_command(SSD1306_COMMAND::SETCONTRAST);
  send_spi_command(0xCF);
  send_spi_command(SSD1306_COMMAND::SETPRECHARGE);
  send_spi_command(0xF1);
  send_spi_command(SSD1306_COMMAND::SETVCOMDETECT);
  send_spi_command(0x40);
  send_spi_command(SSD1306_COMMAND::DISPLAYALLON_RESUME);
  send_spi_command(SSD1306_COMMAND::NORMALDISPLAY);

  send_spi_command(SSD1306_COMMAND::DISPLAYON);
}

void Display::send_spi_command(unsigned char command) {
  digitalWrite(dc_pin, LOW);
  wiringPiSPIDataRW(channel, &command, 1);
}

void Display::send_spi_command(SSD1306_COMMAND command) {
  return send_spi_command((unsigned char)command);
}

void Display::send_spi_data(std::vector<unsigned char> &data) {
  digitalWrite(dc_pin, HIGH);
  wiringPiSPIDataRW(channel, &data[0], data.size());
}

void Display::reset() {
  digitalWrite(rst_pin, HIGH);
  delay(1);
  digitalWrite(rst_pin, LOW);
  delay(10);
  digitalWrite(rst_pin, HIGH);
}

void Display::update(const Bitmap &bitmap) {
  m_map.blit(bitmap, {0, 0});

  send_spi_command(SSD1306_COMMAND::COLUMNADDR);
  send_spi_command(0);                  // Column start address. (0 = reset)
  send_spi_command(bitmap.width() - 1); // Column end address.
  send_spi_command(SSD1306_COMMAND::PAGEADDR);
  send_spi_command(0);                      // Page start address. (0 = reset)
  send_spi_command(bitmap.width() / 8 - 1); // Page end address.

  auto pages = height / 8;
  std::vector<unsigned char> data;

  for (auto page = 0; page < pages; ++page) {
    for (auto x = 0; x < width; ++x) {
      unsigned char bits = 0;
      for (unsigned char bit = 0; bit < 8; ++bit) {
        bits = bits << 1;
        bits |= m_map[page * 8 + 7 - bit][x];
      }
      data.push_back(bits);
    }
  }

  send_spi_data(data);
}
