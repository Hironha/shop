use domain::user;

#[derive(Clone, Debug)]
pub struct LoginInput {
    pub email: user::Email,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct RegisterInput {
    pub username: user::Username,
    pub email: user::Email,
    pub password: String,
}
