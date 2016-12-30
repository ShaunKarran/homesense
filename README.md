# HomeSense

System to record sensor data from different locations in the house and transmit them back to a central server/database.

# TODO
## Server
### v0.1.0
- [ ] Clean up database/diesel code.
- [ ] Dont loop when handling clients. ESP deep sleep means its a new connection for ever reading.
- [ ] Change database design for multiple esps/sensors.
- [ ] Add simple web interface that just dispays the information from the database in text.

## ESP8266
### v0.1.0
- [ ] Buy ESP-01, battery(s), charger and materials for perfboard design.
- [ ] Copy perfboard design from [this](https://tzapu.com/minimalist-battery-powered-esp8266-wifi-temperature-logger/) project.

### v0.2.0
- [ ] Design custom PCB.
- [ ] Buy ESP-12 to use with custom PCB design.

## ESP-Mocker
### v0.1.0
- [ ] Create a new connection for every reading to accurately mock the ESP using deep sleep.
