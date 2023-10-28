#include "LoRa.h"

#define LORA_BAND 868E6

void setup() {
  Serial.begin(115200);
  while (!Serial);
  LoRa.begin(LORA_BAND);
}

void loop() {
  if (Serial.available()) {
    // Get the data from serial
    String receivedData = Serial.readStringUntil('\n');
    int16_t temperature = receivedData.toInt();
    // Send data through LoRa
    LoRa.beginPacket();
    LoRa.write((temperature >> 8) & 0xFF);  // Send the high byte of the temperature
    LoRa.write(temperature & 0xFF);  // Send the low byte of the temperature
    LoRa.endPacket();
    delay(10000);  // 10 sec wait
  }
}
