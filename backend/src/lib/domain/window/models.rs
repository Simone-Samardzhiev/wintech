use crate::domain::window::models::WindowValidationError::InvalidPreferredTemperature;
use thiserror::Error;
use time::Time;
use uuid::Uuid;

/// Possible errors from window validation
#[derive(Error, Debug)]
pub enum WindowValidationError {
    #[error("Invalid window preferred temperature: (min: {min}, max: {max}, actual: {actual})")]
    InvalidPreferredTemperature { min: i16, max: i16, actual: i16 },
}

/// Window related errors.
#[derive(Error, Debug)]
pub enum WindowError {
    #[error("Invalid window")]
    InvalidWindow(Vec<WindowValidationError>),

    #[error("Invalid token")]
    InvalidToken,

    #[error("Unknown error")]
    Unknown(#[from] anyhow::Error),
}

/// Valid preferred temperature.
#[derive(Debug, Copy, Clone)]
pub struct PreferredWindowTemp {
    temp: i16,
}

impl PreferredWindowTemp {
    const MIN: i16 = -5;
    const MAX: i16 = 30;

    fn parse(temp: i16) -> Result<Self, WindowValidationError> {
        if temp < Self::MIN || temp > Self::MAX {
            Err(InvalidPreferredTemperature {
                min: temp,
                max: temp,
                actual: temp,
            })
        } else {
            Ok(PreferredWindowTemp { temp })
        }
    }
}

impl From<PreferredWindowTemp> for i16 {
    fn from(p: PreferredWindowTemp) -> Self {
        p.temp
    }
}

/// Window entity.
#[derive(Debug)]
pub struct Window {
    pub id: Uuid,
    pub preferred_temperature: PreferredWindowTemp,
    pub preferred_wake_up_time: Time,
    pub preferred_bedtime: Time,
}

impl Window {
    pub fn parse(
        id: Uuid,
        preferred_temperature: i16,
        preferred_wake_up_time: Time,
        preferred_bedtime: Time,
    ) -> Result<Self, WindowError> {
        let parsed_temp = PreferredWindowTemp::parse(preferred_temperature)
            .map_err(|e| WindowError::InvalidWindow(vec![e]))?;

        Ok(Self {
            id,
            preferred_temperature: parsed_temp,
            preferred_wake_up_time,
            preferred_bedtime,
        })
    }
}
