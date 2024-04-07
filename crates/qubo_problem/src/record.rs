use std::time::{Duration};
use serde::{Serialize, Serializer};
use crate::QuboType;


fn to_duration_ns<S>(x: &Duration, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    s.serialize_u128(x.as_nanos())
}

#[derive(Serialize)]
pub struct EnergyRecord {
    #[serde(serialize_with = "to_duration_ns")]
    time_stamp: Duration,
    iteration_stamp: usize,
    energy: QuboType
}

impl EnergyRecord {
    pub fn create_with_time(time_stamp: Duration, iteration_stamp: usize, energy: QuboType) -> Self {
        Self {
            time_stamp,
            iteration_stamp,
            energy
        }
    }

    pub fn create(start_time: std::time::Instant, iteration: usize, energy: QuboType) -> Self {
        Self::create_with_time(std::time::Instant::now() - start_time, iteration, energy)
    }
}
