# This file is executed on every boot (including wake-boot from deepsleep)
# import esp
# esp.osdebug(None)
import gc
import webrepl

import wifi
from credentials import SSID, WIFI_PASSWORD


webrepl.start()
gc.collect()

wifi.connect(SSID, WIFI_PASSWORD)
