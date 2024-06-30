use domain::product;

#[derive(Clone, Debug)]
pub struct CreateInput {
    pub catalog_id: String,
    pub name: String,
    pub price: u64,
    pub extras_ids: ExtrasIds,
}

#[derive(Clone, Debug)]
pub struct DeleteInput {
    pub id: String,
    pub catalog_id: String,
}

#[derive(Clone, Debug)]
pub struct FindInput {
    pub id: String,
    pub catalog_id: String,
}

#[derive(Clone, Debug)]
pub struct UpdateInput {
    pub id: String,
    pub catalog_id: String,
    pub name: String,
    pub price: u64,
    pub extras_ids: ExtrasIds,
}

#[derive(Clone, Debug)]
pub struct ExtrasIds(Vec<String>);

impl ExtrasIds {
    pub const MAX_LEN: usize = product::Extras::MAX_LEN;

    pub fn new(ids: Vec<String>) -> Result<Self, product::ExtrasError> {
        if ids.len() > Self::MAX_LEN {
            Err(product::ExtrasError::Length)
        } else {
            Ok(Self(ids))
        }
    }

    pub fn take(self) -> Vec<String> {
        self.0
    }
}
