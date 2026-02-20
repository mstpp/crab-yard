#![allow(dead_code)]
use chrono::{DateTime, Utc};

#[derive(Debug)]
enum VolumeError {
    EmptyName,
}

#[derive(Debug)]
struct VolumeName(String);

impl TryFrom<&str> for VolumeName {
    type Error = VolumeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(VolumeError::EmptyName)
        } else {
            Ok(Self(value.to_owned()))
        }
    }
}

#[derive(Debug)]
struct Volume {
    name: VolumeName,
    size: u64,
    created_at: DateTime<Utc>,
}

impl Volume {
    fn new<S: AsRef<str>>(name: S) -> Result<Self, VolumeError> {
        Ok(Volume {
            name: VolumeName::try_from(name.as_ref())?,
            size: 0,
            created_at: Utc::now(),
        })
    }
}

fn main() {
    let vol_1 = Volume::new("Alice").unwrap();
    println!("{vol_1:?}");
}
