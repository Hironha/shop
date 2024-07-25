use std::num::{NonZeroU32, NonZeroU8};

use domain::catalog;

#[derive(Clone, Debug)]
pub struct CreateInput {
    pub name: catalog::Name,
    pub description: Option<catalog::Description>,
}

#[derive(Clone, Debug)]
pub struct DeleteInput {
    pub id: catalog::Id,
}

#[derive(Clone, Debug)]
pub struct FindInput {
    pub id: catalog::Id,
}

#[derive(Clone, Debug)]
pub struct ListInput {
    pub page: NonZeroU32,
    pub limit: NonZeroU8,
}

#[derive(Clone, Debug)]
pub struct UpdateInput {
    pub id: catalog::Id,
    pub name: catalog::Name,
    pub description: Option<catalog::Description>,
}
