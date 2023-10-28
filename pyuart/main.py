import serial
import urllib.request
import json
import time


SERIAL_PORT = "/dev/serial0"
BAUD_RATE = 115200
API_ENDPOINT = "http://127.0.0.1:8081/temperature"
POLL_INTERVAL = 10


def fetch_temperature():
    try:
        res = urllib.request.urlopen(API_ENDPOINT)
        data = json.loads(res.read().decode("utf-8"))
        temperature = data["t"]["temperature"] / 10  # Converting to full celsius from tenths
        return temperature
    except Exception:
        # Could use some error logging...
        return None


def main():
    with serial.Serial(SERIAL_PORT, BAUD_RATE, timeout=1) as s:
        while True:
            temperature = fetch_temperature()
            if temperature is not None:
                s.write(str(temperature).encode("utf-8"))
            time.sleep(POLL_INTERVAL)


if __name__ == "__main__":
    main()

