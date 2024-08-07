use domain::user;

use super::session;

#[derive(Clone, Debug)]
pub struct LoginInput {
    pub email: user::Email,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct LogoutInput {
    pub session_id: session::Id,
}

#[derive(Clone, Debug)]
pub struct RegisterInput {
    pub username: user::Username,
    pub email: user::Email,
    pub password: String,
}
