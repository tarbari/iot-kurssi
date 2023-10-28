import serial
import urllib.request
import json
import time

s = serial.Serial("/dev/serial0", 115200, timeout=1)  # TODO: Check that the serial dev and baud are actually correct
s.flush()

while True:
    res = json.loads(urllib.request.urlopen("http://127.0.0.1:8081/temperature").read().decode("utf-8"))
    res = res["t"]["temperature"]  # NOTE! These are tenths of celsius!
    s.write(str(res).encode("utf-8"))
    time.sleep(10)
