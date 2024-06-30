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
    pub page: u64,
    pub limit: u64,
}

#[derive(Clone, Debug)]
pub struct UpdateInput {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}
