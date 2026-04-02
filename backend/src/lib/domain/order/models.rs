use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

/// Possible errors from order validation.
#[derive(Debug, Error)]
pub enum OrderValidationError {
    #[error("Invalid postal code length: (min: {min}, max: {max}, actual: {actual})")]
    InvalidPostalCodeNameLength {
        min: usize,
        max: usize,
        actual: usize,
    },

    #[error("Invalid city length: (min: {min}, max: {max}, actual: {actual})")]
    InvalidCityNameLength {
        min: usize,
        max: usize,
        actual: usize,
    },

    #[error("Invalid neighborhood length: (min: {min}, max: {max}, actual: {actual})")]
    InvalidNeighborhoodNameLength {
        min: usize,
        max: usize,
        actual: usize,
    },

    #[error("Invalid street length: (min: {min}, max: {max}, actual: {actual})")]
    InvalidStreetNameLength {
        min: usize,
        max: usize,
        actual: usize,
    },

    #[error("Floor level cannot be less than 0")]
    InvalidFloorLevel,

    #[error("Order delivered time cannot be before creation time")]
    InvalidDeliveryDate,
}

/// Order related errors.
#[derive(Error, Debug)]
pub enum OrderError {
    #[error("Invalid order")]
    InvalidOrder(Vec<OrderValidationError>),

    #[error("Invalid order request")]
    InvalidOrderRequest(Vec<OrderValidationError>),

    #[error("Invalid token")]
    InvalidToken,

    #[error("Unknown error")]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct PostalCode {
    pub code: String,
}

impl PostalCode {
    const MIN_LENGTH: usize = 3;
    const MAX_LENGTH: usize = 12;
    fn parse(mut code: String) -> Result<Self, OrderValidationError> {
        code = code.trim().to_string();
        let length = code.chars().count();
        if length < Self::MIN_LENGTH || length > Self::MAX_LENGTH {
            return Err(OrderValidationError::InvalidPostalCodeNameLength {
                min: Self::MIN_LENGTH,
                max: Self::MAX_LENGTH,
                actual: length,
            });
        }

        Ok(Self { code })
    }
}

impl AsRef<str> for PostalCode {
    fn as_ref(&self) -> &str {
        &self.code
    }
}
impl From<PostalCode> for String {
    fn from(code: PostalCode) -> String {
        code.code
    }
}

/// Valid city.
#[derive(Debug)]
pub struct City {
    name: String,
}

impl City {
    const MIN_LENGTH: usize = 3;
    const MAX_LENGTH: usize = 100;
    pub fn parse(mut name: String) -> Result<Self, OrderValidationError> {
        name = name.trim().to_string();
        let length = name.chars().count();

        if length < Self::MIN_LENGTH || length > Self::MAX_LENGTH {
            return Err(OrderValidationError::InvalidCityNameLength {
                min: Self::MIN_LENGTH,
                max: Self::MAX_LENGTH,
                actual: length,
            });
        }

        Ok(Self { name })
    }
}

impl AsRef<str> for City {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl From<City> for String {
    fn from(city: City) -> String {
        city.name
    }
}

/// Valid neighborhood.
#[derive(Debug)]
pub struct Neighborhood {
    name: String,
}

impl Neighborhood {
    const MIN_LENGTH: usize = 3;
    const MAX_LENGTH: usize = 100;
    pub fn parse(neighborhood: Option<String>) -> Result<Option<Self>, OrderValidationError> {
        let mut name = match neighborhood {
            Some(name) => name,
            None => return Ok(None),
        };

        name = name.trim().to_string();
        let length = name.chars().count();

        if length < Self::MIN_LENGTH || length > Self::MAX_LENGTH {
            return Err(OrderValidationError::InvalidNeighborhoodNameLength {
                min: Self::MIN_LENGTH,
                max: Self::MAX_LENGTH,
                actual: length,
            });
        }

        Ok(Some(Self { name }))
    }
}

impl AsRef<str> for Neighborhood {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl From<Neighborhood> for String {
    fn from(neighborhood: Neighborhood) -> String {
        neighborhood.name
    }
}

/// Valid street.
#[derive(Debug)]
pub struct Street {
    name: String,
}

impl Street {
    const MIN_LENGTH: usize = 3;
    const MAX_LENGTH: usize = 100;
    pub fn parse(mut name: String) -> Result<Self, OrderValidationError> {
        name = name.trim().to_string();
        let length = name.chars().count();

        if length < Self::MIN_LENGTH || length > Self::MAX_LENGTH {
            return Err(OrderValidationError::InvalidStreetNameLength {
                min: Self::MIN_LENGTH,
                max: Self::MAX_LENGTH,
                actual: length,
            });
        }

        Ok(Self { name })
    }
}

impl AsRef<str> for Street {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl From<Street> for String {
    fn from(street: Street) -> String {
        street.name
    }
}

/// Valid floor level.
#[derive(Clone, Copy, Debug)]
pub struct FloorLevel {
    level: i16,
}

impl FloorLevel {
    pub fn parse(level: Option<i16>) -> Result<Option<Self>, OrderValidationError> {
        let level = match level {
            Some(level) => level,
            None => return Ok(None),
        };

        if level < 0 {
            return Err(OrderValidationError::InvalidFloorLevel);
        }

        Ok(Some(Self { level }))
    }
}

impl From<FloorLevel> for i16 {
    fn from(level: FloorLevel) -> Self {
        level.level
    }
}

/// Order entity.
#[derive(Debug)]
pub struct Order {
    pub id: Uuid,
    pub postal_code: PostalCode,
    pub city: City,
    pub neighborhood: Option<Neighborhood>,
    pub street: Street,
    pub floor_level: Option<FloorLevel>,
    pub created_at: OffsetDateTime,
    pub delivered_at: Option<OffsetDateTime>,
    pub user_id: Uuid,
}

impl Order {
    pub fn parse(
        id: Uuid,
        postal_code: String,
        city: String,
        neighborhood: Option<String>,
        street: String,
        floor_level: Option<i16>,
        created_at: OffsetDateTime,
        delivered_at: Option<OffsetDateTime>,
        user_id: Uuid,
    ) -> Result<Self, OrderError> {
        let mut errs: Vec<OrderValidationError> = Vec::new();

        let postal_code = PostalCode::parse(postal_code)
            .map_err(|e| errs.push(e))
            .ok();
        let city = City::parse(city).map_err(|e| errs.push(e)).ok();
        let neighborhood = Neighborhood::parse(neighborhood)
            .map_err(|e| errs.push(e))
            .ok();
        let street = Street::parse(street).map_err(|e| errs.push(e)).ok();
        let floor_level = FloorLevel::parse(floor_level)
            .map_err(|e| errs.push(e))
            .ok();

        if let Some(t) = delivered_at {
            if t < created_at {
                errs.push(OrderValidationError::InvalidDeliveryDate)
            }
        }

        if !errs.is_empty() {
            return Err(OrderError::InvalidOrder(errs));
        }

        Ok(Self {
            id,
            postal_code: postal_code.unwrap(),
            city: city.unwrap(),
            neighborhood: neighborhood.unwrap(),
            street: street.unwrap(),
            floor_level: floor_level.unwrap(),
            created_at,
            delivered_at,
            user_id,
        })
    }

    pub fn new(
        id: Uuid,
        postal_code: PostalCode,
        city: City,
        neighborhood: Option<Neighborhood>,
        street: Street,
        floor_level: Option<FloorLevel>,
        created_at: OffsetDateTime,
        delivered_at: Option<OffsetDateTime>,
        user_id: Uuid,
    ) -> Self {
        Self {
            id,
            postal_code,
            city,
            neighborhood,
            street,
            floor_level,
            created_at,
            delivered_at,
            user_id,
        }
    }
}

/// Request to place new order.
#[derive(Debug)]
pub struct OrderRequest {
    pub postal_code: PostalCode,
    pub city: City,
    pub neighborhood: Option<Neighborhood>,
    pub street: Street,
    pub floor_level: Option<FloorLevel>,
}

impl OrderRequest {
    pub fn parse(
        postal_code: String,
        city: String,
        neighborhood: Option<String>,
        street: String,
        floor_level: Option<i16>,
    ) -> Result<Self, OrderError> {
        let mut errs: Vec<OrderValidationError> = Vec::new();

        let postal_code = PostalCode::parse(postal_code)
            .map_err(|e| errs.push(e))
            .ok();
        let city = City::parse(city).map_err(|e| errs.push(e)).ok();
        let neighborhood = Neighborhood::parse(neighborhood)
            .map_err(|e| errs.push(e))
            .ok();
        let street = Street::parse(street).map_err(|e| errs.push(e)).ok();
        let floor_level = FloorLevel::parse(floor_level)
            .map_err(|e| errs.push(e))
            .ok();

        if !errs.is_empty() {
            return Err(OrderError::InvalidOrder(errs));
        }

        Ok(Self {
            postal_code: postal_code.unwrap(),
            city: city.unwrap(),
            neighborhood: neighborhood.unwrap(),
            street: street.unwrap(),
            floor_level: floor_level.unwrap(),
        })
    }
}
