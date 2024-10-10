
CREATE TABLE devices (
    beamline TEXT NOT NULL,
    device_name TEXT NOT NULL,
    uuid INTEGER NOT NULL,
    PRIMARY KEY (beamline, device_name),
)


CREATE TABLE insertion_device (
    uuid INTEGER PRIMARY KEY AUTOINCREMENT,
    poles INTEGER,
    length REAL,
)


-- Test Data
INSERT INTO insertion_device (poles, length) VALUES (80,2.0);

uuid = last_insert_rowid();
INSERT INTO devices (beamline, device_name, uuid) VALUES ('i22', 'undulator', uuid)
