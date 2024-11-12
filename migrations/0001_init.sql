CREATE TABLE devices (
    beamline TEXT NOT NULL,
    device_name TEXT NOT NULL,
    uuid INTEGER NOT NULL,
    PRIMARY KEY (beamline, device_name)
);

-- NXinsertion_device
/* https://manual.nexusformat.org/classes/base_classes/NXinsertion_device.html#nxinsertion-device */
CREATE TABLE insertion_device (
    uuid INTEGER PRIMARY KEY AUTOINCREMENT,
    "default" CHAR,
    type CHAR,
    gap FLOAT,
    taper FLOAT,
    phase FLOAT,
    poles INTEGER,
    magnetic_wavelength FLOAT,
    k FLOAT,
    length REAL,
    power FLOAT,
    energy FLOAT,
    bandwidth FLOAT,
    harmonic INT,
    depends_on CHAR
    -- spectrum NXdata,
    -- OFF_GEOMETRY NXoff_geometry,
    -- TRANSFORMATIONS NXtransformations,
);

-- Test Data
INSERT INTO insertion_device (poles, length) VALUES (1,1.0), (2,2.0);
INSERT INTO devices (beamline, device_name, uuid) VALUES ('i22', 'undulator', 1), ('i15','wiggler', 2)
