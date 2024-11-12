use async_graphql::{Enum, SimpleObject};
use sqlx::FromRow;

// NeXus definitions

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum InsertionDeviceType {
    Undulator,
    Wiggler,
}

type Length = f64;
type Angle = f64;
type Power = f64;
type Energy = f64;

#[derive(SimpleObject, Clone, FromRow, Debug)]
pub struct InsertionDevice {
    pub default: Option<String>,
    pub r#type: Option<String>,
    pub gap: Option<Length>,
    pub taper: Option<Angle>,
    pub phase: Option<Angle>,
    pub poles: Option<i32>,
    pub magnetic_wavelength: Option<Length>,
    pub k: Option<f64>,
    pub length: Option<Length>,
    pub power: Option<Power>,
    pub energy: Option<Energy>,
    pub bandwidth: Option<Energy>,
    pub harmonic: Option<i32>,
    pub depends_on: Option<String>,
}

#[derive(SimpleObject, Clone, FromRow, Debug)]
pub struct Devices {
    pub beamline: String,
    pub device_name: String,
    pub uuid: i64,
}
