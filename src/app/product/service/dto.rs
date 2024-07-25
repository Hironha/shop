use thiserror::Error;

use domain::catalog;
use domain::extra;
use domain::product;

#[derive(Clone, Debug)]
pub struct CreateInput {
    pub catalog_id: catalog::Id,
    pub name: product::Name,
    pub price: product::Price,
    pub kind: product::Kind,
    pub extras_ids: ExtrasIds,
}

#[derive(Clone, Debug)]
pub struct DeleteInput {
    pub id: product::Id,
    pub catalog_id: catalog::Id,
}

#[derive(Clone, Debug)]
pub struct FindInput {
    pub id: product::Id,
    pub catalog_id: catalog::Id,
}

#[derive(Clone, Debug)]
pub struct UpdateInput {
    pub id: product::Id,
    pub catalog_id: catalog::Id,
    pub name: product::Name,
    pub price: product::Price,
    pub kind: product::Kind,
    pub extras_ids: ExtrasIds,
}

#[derive(Clone, Debug)]
pub struct ExtrasIds(Vec<extra::Id>);

impl ExtrasIds {
    pub const MAX_LEN: usize = product::Extras::MAX_LEN;

    pub fn parse(ids: &[String]) -> Result<Self, ParseExtrasIdsError> {
        if ids.len() > Self::MAX_LEN {
            return Err(ParseExtrasIdsError::Length);
        }

        ids.iter()
            .map(|id| extra::Id::parse_str(id).map_err(|_| ParseExtrasIdsError::Invalid(id)))
            .collect::<Result<Vec<_>, ParseExtrasIdsError>>()
            .map(Self)
    }
}

impl ExtrasIds {
    pub fn as_slice(&self) -> &[extra::Id] {
        &self.0
    }

    pub fn take(self) -> Vec<extra::Id> {
        self.0
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseExtrasIdsError<'a> {
    #[error("Extras ids cannot have more than {len} items", len = ExtrasIds::MAX_LEN)]
    Length,
    #[error("Provided extra id `{0}` is not a valid")]
    Invalid(&'a str),
}
