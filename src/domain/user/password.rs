use std::fmt;

pub trait Encrypter: Encrypt {
    fn verify(&self, encrypted: &str, raw: &str) -> bool;
}

pub trait Encrypt {
    fn encrypt(&self, password: &str) -> String;
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn new(password: &str, encrypt: &impl Encrypt) -> Self {
        Self(encrypt.encrypt(password))
    }
}

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
