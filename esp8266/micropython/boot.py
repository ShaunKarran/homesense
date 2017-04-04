# This file is executed on every boot (including wake-boot from deepsleep)
# import esp
# esp.osdebug(None)
import gc
import webrepl

import wifi
import wifi_credentials


webrepl.start()
gc.collect()

wifi.connect(wifi_credentials.SSID, wifi_credentials.PASSWORD)
