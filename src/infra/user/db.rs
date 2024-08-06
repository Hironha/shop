use std::sync::{Arc, Mutex};

use domain::user;
use domain::user::password::Password;

#[derive(Clone, Debug, Default)]
pub struct InMemUsers {
    users: Arc<Mutex<Vec<(user::User, Password)>>>,
}

impl InMemUsers {
    pub fn new() -> Self {
        Self::default()
    }
}

impl user::Repository for InMemUsers {
    async fn create(&mut self, user: &user::User, password: &Password) -> Result<(), user::Error> {
        let Ok(mut users) = self.users.try_lock() else {
            return Err(user::Error::any("Failed acquiring users lock"));
        };

        if users.iter().any(|u| u.0.id() == user.id()) {
            return Err(user::Error::id_conflict(user.id()));
        }
        if users.iter().any(|u| u.0.email == user.email) {
            return Err(user::Error::email_conflict(user.email.clone()));
        }

        users.push((user.clone(), password.clone()));
        Ok(())
    }

    async fn find_password(&self, email: &user::Email) -> Result<String, user::Error> {
        let Ok(users) = self.users.try_lock() else {
            return Err(user::Error::any("Failed acquiring users lock"));
        };
        let Some(found) = users.iter().find(|u| &u.0.email == email) else {
            return Err(user::Error::email_not_found(email.clone()));
        };

        Ok(found.1.to_string())
    }
}
