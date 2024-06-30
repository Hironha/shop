#[derive(Clone, Debug)]
pub struct CreateInput {
    pub name: String,
    pub price: u64,
}

#[derive(Clone, Debug)]
pub struct DeleteInput {
    pub id: String,
}

pub struct UpdateInput {
    pub id: String,
    pub name: String,
    pub price: u64,
}
