use std::fmt;

use uuid::Uuid;

use domain::user;

#[allow(clippy::unused_async)]
pub trait Manager {
    async fn create(&mut self, user: &user::User) -> Option<Id>;
    async fn refresh(&mut self, id: Id);
    async fn validate(&self, id: Id) -> bool;
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Id {
    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
