use std::num::{NonZeroU32, NonZeroU8};

#[derive(Clone, Debug)]
pub struct CreateInput {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DeleteInput {
    pub id: String,
}

#[derive(Clone, Debug)]
pub struct FindInput {
    pub id: String,
}

#[derive(Clone, Debug)]
pub struct ListInput {
    pub page: NonZeroU32,
    pub limit: NonZeroU8,
}

#[derive(Clone, Debug)]
pub struct UpdateInput {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}
