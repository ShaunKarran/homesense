import network


def connect(ssid, password):
    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)

    if not wlan.isconnected():
        print("")
        print("")
        print("INFO: Connecting to {}".format(ssid))
        wlan.connect(ssid, password)

        while not wlan.isconnected():
            pass

    print("")
    print("INFO: WiFi connected")
    print('INFO: Network Config = ', wlan.ifconfig())


def disconnect():
    wlan = network.WLAN(network.STA_IF)
    wlan.disconnect()
