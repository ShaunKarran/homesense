# HomeSense

System to record sensor data from different locations in the house and transmit them back to a central server/database.

## TODO
### Server
#### v0.1.0
- [ ] Clean up database/diesel code.
- [ ] Dont loop when handling clients. ESP deep sleep means its a new connection for ever reading.
- [ ] Change database design for multiple esps/sensors.
- [ ] Add simple web interface that just dispays the information from the database in text.

### ESP8266
#### v0.1.0
- [ ] Buy materials.
  - [ ] ESP-01
  - [ ] Battery & charger
  - [ ] HT7333 LDO
  - [ ] DS18B20 Thermometer
  - [x] Perf board
  - [ ] Surface mount capacitors (10uF & maybe 100uF)
  - [ ] Surface mount resistors (4.7kOhm)
- [ ] Copy perfboard design from [this](https://tzapu.com/minimalist-battery-powered-esp8266-wifi-temperature-logger/) project.
- [ ] Add battery voltage measurement, notification of charging needed and shutdown at certain voltage.

#### v0.2.0
- [ ] Design custom PCB.
- [ ] Buy ESP-12 to use with custom PCB design.
- [ ] Allow different sensor reading and connecting to WiFi intervals. Could be tricky to record time of readings, but would reduce power consumption.

### ESP-Mocker
#### v0.1.0
- [ ] Create a new connection for every reading to accurately mock the ESP using deep sleep.
