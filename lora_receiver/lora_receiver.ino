#include "LoRa.h"

#define LORA_BAND  868E6

void setup() {
    Serial.begin(115200);
    while (!Serial);
    if (!LoRa.begin(LORA_BAND)) {
        Serial.println("Starting LoRa failed!");
        while (1);  // If there is an error in setting up the connection, halt execution
    }
}

void loop() {
    int packetSize = LoRa.parsePacket();
    if (packetSize) {
        int16_t temperature = (LoRa.read() << 8) | LoRa.read();  // Read high byte and low byte and recombine them to get the signed int16
        Serial.println("Received temperature: " + String(temperature));
    }
}