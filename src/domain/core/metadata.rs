use thiserror::Error;
use time::OffsetDateTime;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Metadata {
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl Metadata {
    #[must_use]
    pub fn new() -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            created_at: now,
            updated_at: now,
        }
    }

    pub fn configured(
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    ) -> Result<Self, ConfigMetadataError> {
        if created_at > updated_at {
            return Err(ConfigMetadataError);
        }

        Ok(Self {
            created_at,
            updated_at,
        })
    }
}

impl Metadata {
    #[must_use]
    pub fn created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    #[must_use]
    pub fn updated_at(&self) -> OffsetDateTime {
        self.updated_at
    }

    pub fn update(&mut self) {
        self.updated_at = OffsetDateTime::now_utc();
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Metadata created at cannot be bigger than updated at")]
pub struct ConfigMetadataError;
