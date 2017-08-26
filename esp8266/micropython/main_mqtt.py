import dht
import machine
from umqtt.simple import MQTTClient

import credentials
import wifi


MQTT_SENSOR_TOPIC = b"shaun_bedroom/sensor"

SLEEPING_TIME_IN_SECONDS = 900  # 15 minutes * 60 seconds

# DHT - D1/GPIO5
DHTPIN = 5

dht_sensor = dht.DHT22(machine.Pin(DHTPIN))

wifi.connect(credentials.WIFI_SSID, credentials.WIFI_PASSWORD)

mqtt_client = MQTTClient(
    credentials.MQTT_CLIENT_ID,
    credentials.MQTT_SERVER_IP,
    credentials.MQTT_SERVER_PORT,
    credentials.MQTT_USER,
    credentials.MQTT_PASSWORD,
)

print("INFO: Attempting MQTT connection")
mqtt_client.connect()
print("INFO: MQTT connected")

dht_sensor.measure()
humidity = dht_sensor.humidity()
temperature = dht_sensor.temperature()

if humidity is None or temperature is None:
    print("ERROR: Failed to read from DHT sensor!")
else:
    # print(humidity);
    # print(temperature);
    data = {"temperature": temperature, "humidity": humidity}
    print("INFO: data = {}".format(data))
    mqtt_client.publish(MQTT_SENSOR_TOPIC, bytes(str(data), "utf-8"), retain=True)

print("INFO: Closing the MQTT connection")
mqtt_client.disconnect()

print("INFO: Closing the Wifi connection")
wifi.disconnect()

# configure RTC.ALARM0 to be able to wake the device
rtc = machine.RTC()
rtc.irq(trigger=rtc.ALARM0, wake=machine.DEEPSLEEP)

# Set RTC.ALARM0 to fire, waking the device. Argument is in micro seconds. Multiply by 1e6 for seconds.
print("INFO: Entering deepsleep")
rtc.alarm(rtc.ALARM0, SLEEPING_TIME_IN_SECONDS * 1000000)
machine.deepsleep()
