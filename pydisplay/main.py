import board
import displayio
import adafruit_ssd1306
import urllib.request
import json
import time


displayio.release_displays()

i2c = board.I2C()  # RPi pinnit 2 = SDA ja 3 = SCL

display_width = 128
display_height = 64
display = adafruit_ssd1306.SSD1306_I2C(display_width, display_height, i2c)

while True:
    temp = json.loads(urllib.request.urlopen("http://127.0.0.1:8081/temperature").read().decode("utf-8"))
    temp = temp["t"]["temperature"] / 10
    display.fill(0)
    display.text(f"Temperature: {temp}", 0, 0, 1)
    display.show()
    time.sleep(10)
