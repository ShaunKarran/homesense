import network


def connect(ssid, password):
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)

    if not wlan.isconnected():
        print('Connecting to network...')
        wlan.connect(ssid, password)

        while not wlan.isconnected():
            pass

    print('Network Config:', wlan.ifconfig())
