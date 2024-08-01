use domain::extra;

#[derive(Clone, Debug)]
pub struct CreateInput {
    pub name: extra::Name,
    pub price: extra::Price,
}

#[derive(Clone, Debug)]
pub struct DeleteInput {
    pub id: extra::Id,
}

#[derive(Clone, Debug)]
pub struct FindInput {
    pub id: extra::Id,
}

pub struct UpdateInput {
    pub id: extra::Id,
    pub name: extra::Name,
    pub price: extra::Price,
}
