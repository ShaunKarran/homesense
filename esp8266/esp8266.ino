#include <ESP8266WiFi.h>
#include <WiFiClient.h>

#include <DallasTemperature.h>
#include <OneWire.h>


// Stores the SSID and PASSWORD. eg.
// const char* SSID = "My WiFi";
// const char* PASSWORD = "mypassword";
#include "wifi_credentials.h"

// Must be unique for every ESP connecting to the same server.
const uint8_t DEVICE_ID = 0;

// IP and port to connect and send data.
const char* HOST = "192.168.1.12";
const uint16_t PORT = 12345;

// Time to sleep (in seconds).
const uint16_t SECONDS_TO_SLEEP = 10;

// Data wire is plugged into pin D1 on the ESP.
const uint8_t ONE_WIRE_BUS = D1;

OneWire oneWire(ONE_WIRE_BUS); // Setup a oneWire instance to communicate with any OneWire devices.
DallasTemperature sensors(&oneWire); // Pass our oneWire reference to Dallas Temperature.


void setup() {
    Serial.begin(115200);
    WiFi.begin(SSID, PASSWORD);

    // Wait for connection
    while (WiFi.status() != WL_CONNECTED) {
        delay(500);
        Serial.print(".");
    }
    
    Serial.println("");
    Serial.print("Connected to ");
    Serial.println(SSID);
    Serial.print("IP address: ");
    Serial.println(WiFi.localIP());
    Serial.println("");

    // Start up the library
    sensors.begin();
}

void loop() {
    WiFiClient client;
    
    if (client.connect(HOST, PORT)) {
        Serial.println("Connected to the host!");
        
        Serial.print("Requesting temperatures from the thermometer... ");
        sensors.requestTemperaturesByIndex(0);
        Serial.println("DONE.");

        float temperature = sensors.getTempCByIndex(0);
        if (temperature == 85.0) {
            Serial.println("Error communicating with thermometer.");
        } else if (temperature == 127.0 || temperature == -127.00) {
            Serial.println("Thermometer error.");
        } else {
            String data = create_json(temperature);
            Serial.print("Sending data: ");
            Serial.println(data);

            Serial.println("Sending data to the server.");
            client.print(data);
        }
    } else {
        Serial.println("Connection failed!");
    }

    client.stop();
    Serial.println("Disconnected from host.");

    Serial.println("Going to sleep.\n");
    delay(100);
    ESP.deepSleep(SECONDS_TO_SLEEP * 1e6); // Argument is in micro seconds. Multiply by 1e6 for seconds.
}

String create_json(float temperature) {
    String json = "{";
    json += "device_id: ";
    json += DEVICE_ID;
    json += ", ";
    json += "temperature: ";
    json += temperature;
    json += "}";
    json += "\n";

    return json;
}

