CREATE TABLE rooms (
    id SERIAL PRIMARY KEY,
    name VARCHAR(40) UNIQUE
);

CREATE TABLE devices (
    id SERIAL PRIMARY KEY,
    room SERIAL REFERENCES Rooms(id)
);

CREATE TABLE readings (
    id SERIAL PRIMARY KEY,
    recorded_at TIMESTAMP,
    device_id SERIAL REFERENCES Devices(id),
    temperature REAL,
    humidity REAL,
    light REAL
);
