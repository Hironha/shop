use std::fmt;

pub trait Encrypt {
    fn encrypt(&self, password: &str) -> String;
    fn verify(&self, hashed: &str, pwd: &str) -> bool;
}

#[derive(Clone, Debug)]
pub struct PasswordEncrypter<T> {
    encrypter: T,
}

impl<T: Encrypt> PasswordEncrypter<T> {
    pub fn new(encrypter: T) -> Self {
        Self { encrypter }
    }
}

impl<T: Encrypt> PasswordEncrypter<T> {
    pub fn encrypt(&self, password: &str) -> Password {
        Password(self.encrypter.encrypt(password))
    }

    pub fn verify(&self, hashed: &str, password: &str) -> bool {
        self.encrypter.verify(hashed, password)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Password(String);

impl Password {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
