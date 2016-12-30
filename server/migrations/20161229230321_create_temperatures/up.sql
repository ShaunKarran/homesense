CREATE TABLE temperatures (
    id SERIAL PRIMARY KEY,
    recorded_at TIMESTAMP NOT NULL,
    temperature REAL NOT NULL
)
