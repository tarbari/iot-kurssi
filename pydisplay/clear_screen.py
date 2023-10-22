import board
import displayio
import adafruit_ssd1306


displayio.release_displays()

# Create the I2C bus interface.
i2c = board.I2C()  # uses board.SCL and board.SDA

# Create the SSD1306 OLED class.
display_width = 128
display_height = 64
display = adafruit_ssd1306.SSD1306_I2C(display_width, display_height, i2c)
display.fill(0)
