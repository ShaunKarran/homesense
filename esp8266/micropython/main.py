import esp
import machine
import socket

from credentials import EMONCMS_WRITE_KEY
import esp_homesense

# Configure the onboard LED and set it to off.
LED = machine.Pin(2, machine.Pin.OUT)
LED.high()

# Configure RTC.ALARM0 to be able to wake the device.
rtc = machine.RTC()
rtc.irq(trigger=rtc.ALARM0, wake=machine.DEEPSLEEP)

# Check if the device woke from a deep sleep.
if machine.reset_cause() != machine.DEEPSLEEP_RESET:
    # TODO Possibly use a non-deepsleep wake as a way to make the device wait so I can use the REPL.
    print('Did not wake from deepsleep.')
else:
    print('Woke from deepsleep.')

host = 'emoncms.org'

device_id = esp.flash_id()

temperature = esp_homesense.read_temperature()
print('Temperature: {}'.format(temperature))

# TODO Copied from internet. Not sure if working.
vcc = machine.ADC(1)
voltage = vcc.read()

json = '{{temperature:{temp}}}'.format(temp=temperature)
url = '/input/post.json?node={}&apikey={}&json={}'.format(device_id, EMONCMS_WRITE_KEY, json)
message = 'GET {url} HTTP/1.1\r\nHost: {host}\r\n\r\n'.format(url=url, host=host)
print('Message: {}'.format(bytes(message, 'utf8')))

# connection = http_client.HTTPConnection(host)
# connection.request('GET', url)
# response = connection.getresponse()
# print(response.read())

http_port = 80
addr = socket.getaddrinfo(host, 80)[0][-1]
s = socket.socket()
s.connect(addr)
s.send(bytes(message, 'utf8'))
s.close()

# import time; time.sleep(60)  # Hopefully give me a change to get into REPL for debugging.

# Set RTC.ALARM0 to fire after 10 seconds (waking the device).
# print('Entering deepsleep.')
# rtc.alarm(rtc.ALARM0, 10 * 1000)  # 10s * 1000ms
# machine.deepsleep()
