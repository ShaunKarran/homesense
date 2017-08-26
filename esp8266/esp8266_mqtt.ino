#include <ArduinoJson.h>
#include <ESP8266WiFi.h>
#include <PubSubClient.h>

#include "DHT.h"

// Stores secret values, SSID, PASSWORD, etc.
// e.g.
// const char* WIFI_SSID = "My WiFi";
// const char* WIFI_PASSWORD = "mypassword";
#include "credentials.h"

#define MQTT_VERSION MQTT_VERSION_3_1_1

const PROGMEM char* MQTT_SENSOR_TOPIC = "shaun_bedroom/sensor";

const PROGMEM uint16_t SLEEPING_TIME_IN_SECONDS = 900; // 15 minutes x 60 seconds

// DHT - D1/GPIO5
#define DHTPIN 5
#define DHTTYPE DHT22

DHT dht(DHTPIN, DHTTYPE);
WiFiClient wifi_client;
PubSubClient mqtt_client(wifi_client);

// Function called when a MQTT message arrived.
void callback(char* p_topic, byte* p_payload, unsigned int p_length) {
    // Do nothing.
}

void wifi_connect(char* ssid, char* password) {
    Serial.println("");
    Serial.println("");
    Serial.print("INFO: Connecting to ");
    Serial.println(ssid);
    WiFi.mode(WIFI_STA);
    WiFi.begin(ssid, password);

    while (WiFi.status() != WL_CONNECTED) {
        delay(500);
        Serial.print(".");
    }

    Serial.println("");
    Serial.println("INFO: WiFi connected");
    Serial.println("INFO: IP address: ");
    Serial.println(WiFi.localIP());
}

// Function called to publish the temperature and the humidity.
void publish_data(float temperature, float humidity) {
    // Create a JSON object. Docs: https://github.com/bblanchon/ArduinoJson/wiki/API%20Reference
    StaticJsonBuffer<200> json_buffer;
    JsonObject& root = json_buffer.createObject();
    // INFO: the data must be converted into a string; a problem occurs when using floats...
    root["temperature"] = (String)temperature;
    root["humidity"] = (String)humidity;
    root.prettyPrintTo(Serial);
    Serial.println("");
    /*
    {
        "temperature": "23.20",
        "humidity": "43.70"
    }
    */
    char data[200];
    root.printTo(data, root.measureLength() + 1);
    client.publish(MQTT_SENSOR_TOPIC, data, true);
}

// Only use the setup function because using deep sleep means the device is restarted.
void setup() {
    // Initialise the serial connection. For logging messages.
    Serial.begin(115200);

    // Initialise the temperature sensor.
    dht.begin();

    // Initialise the WiFi connection.
    wifi_connect(WIFI_SSID, WIFI_PASSWORD);

    // Initialise the MQTT connection.
    mqtt_client.setServer(MQTT_SERVER_IP, MQTT_SERVER_PORT);
    mqtt_client.setCallback(callback);

    // Loop until we're connected.
    Serial.println("");
    while (!mqtt_client.connected()) {
        Serial.print("INFO: Attempting MQTT connection");
        // Attempt to connect
        if (mqtt_client.connect(MQTT_CLIENT_ID, MQTT_USER, MQTT_PASSWORD)) {
            Serial.println("INFO: MQTT connected");
        } else {
            Serial.print("ERROR: Failed, rc=");
            Serial.print(mqtt_client.state());
            Serial.println("DEBUG: Try again in 5 seconds");
            // Wait 5 seconds before retrying
            delay(5000);
        }
    }
    mqtt_client.loop();

    // Reading temperature or humidity takes about 250 milliseconds!
    // Sensor readings may also be up to 2 seconds 'old' (its a very slow sensor).
    float humidity = dht.readHumidity();
    // Read temperature as Celsius (the default).
    float temperature = dht.readTemperature();

    if (isnan(humidity) || isnan(temperature)) {
        Serial.println("ERROR: Failed to read from DHT sensor!");
    } else {
        // Serial.println(humidity);
        // Serial.println(temperature);
        publish_data(temperature, humidity);
    }

    Serial.println("INFO: Closing the MQTT connection");
    mqtt_client.disconnect();

    Serial.println("INFO: Closing the Wifi connection");
    WiFi.disconnect();

    // Argument is in micro seconds. Multiply by 1e6 for seconds.
    Serial.println("INFO: Entering deepsleep");
    ESP.deepSleep(SLEEPING_TIME_IN_SECONDS * 1000000, WAKE_RF_DEFAULT);
    delay(500); // wait for deep sleep to happen
}
