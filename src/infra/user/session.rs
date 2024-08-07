use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::app::user::service::session;

use domain::user;

#[derive(Clone, Debug, Default)]
pub struct InMemSessions {
    sessions: Arc<Mutex<HashMap<session::Id, user::User>>>,
}

impl InMemSessions {
    pub fn new() -> Self {
        Self::default()
    }
}

impl session::Manager for InMemSessions {
    async fn create(&mut self, user: &user::User) -> Option<session::Id> {
        let mut sessions = self.sessions.try_lock().ok()?;
        let id = session::Id::new();
        sessions.insert(id, user.clone());

        Some(id)
    }

    async fn refresh(&mut self, _id: session::Id) {
        todo!()
    }

    async fn revoke(&mut self, id: session::Id) {
        let mut sessions = self.sessions.try_lock().unwrap();
        sessions.remove(&id);
    }

    async fn validate(&self, id: session::Id) -> bool {
        let Ok(sessions) = self.sessions.try_lock() else {
            return false;
        };
        sessions.contains_key(&id)
    }
}
