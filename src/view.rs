use serde::{Deserialize, Serialize};

/// All native readings are optional. Missing hardware is never reported as zero.
/// The owner publishes percentages and rates over its own sampling interval.
#[derive(Clone, Deserialize, Serialize)]
pub struct SystemSnapshot {
    pub v: u8,
    pub cpu: Cpu,
    pub memory: Memory,
    pub gpu: Gpu,
    pub disk: Disk,
    pub net: Network,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Cpu {
    #[serde(deserialize_with = "Option::deserialize")]
    pub usage: Option<f64>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub temp: Option<f64>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Memory {
    #[serde(deserialize_with = "Option::deserialize")]
    pub used: Option<f64>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub swap: Option<f64>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Gpu {
    #[serde(deserialize_with = "Option::deserialize")]
    pub usage: Option<f64>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub memory: Option<f64>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub temp: Option<f64>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Disk {
    #[serde(deserialize_with = "Option::deserialize")]
    pub read: Option<u64>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub write: Option<u64>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub temp: Option<f64>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Network {
    #[serde(deserialize_with = "Option::deserialize")]
    pub rx: Option<u64>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub tx: Option<u64>,
}
