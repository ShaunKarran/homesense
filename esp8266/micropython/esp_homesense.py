import ds18x20
from machine import Pin
import onewire
import time

ow = onewire.OneWire(Pin(4))  # Wemos D1 mini pin D2
ds_sensor = ds18x20.DS18X20(ow)


def read_temperature():
    """
    Read the temperature from a DS18X20 device.

    :return: The current temperature in degrees Celsius.
    """
    roms = ds_sensor.scan()
    if len(roms) == 0:
        # TODO TempSensorError
        raise ValueError('ds18x20.scan() returned no results. Check the thermometer is connected.')
    rom = roms[0]  # I dont currently know what multiple roms would mean. Multiple physical sensors on 1 wire?

    ds_sensor.convert_temp()
    time.sleep_ms(750)  # TODO try smaller values. Its from example, I assume its required by the sensor.

    temperature = ds_sensor.read_temp(rom)

    if temperature == 85.0:
        raise ValueError('Error communicating with thermometer.')

    if temperature in [-127.0, 127.0]:
        raise ValueError('Thermometer failed to read temperature.')

    return temperature
