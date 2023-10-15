use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temperature {
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bounds {
    pub lower: f32,
    pub upper: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TempAndBounds {
    pub t: Temperature,
    pub b: Bounds,
}
