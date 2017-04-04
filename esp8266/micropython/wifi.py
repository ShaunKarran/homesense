import network

IP_ADDRESS = '192.168.1.16'
SUBNET_MASK = '255.255.255.0'
GATEWAY = '192.168.1.1'
DNS = '8.8.8.8'


def connect(ssid, password):
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)

    wlan.ifconfig([IP_ADDRESS, SUBNET_MASK, GATEWAY, DNS])

    if not wlan.isconnected():
        print('Connecting to network...')
        wlan.connect(ssid, password)

        while not wlan.isconnected():
            pass

    print('Network Config:', wlan.ifconfig())
