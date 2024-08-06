use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordVerifier, Version};

use domain::user::password::{Encrypt, Encrypter};

#[derive(Clone, Debug)]
pub struct Argon2Encrypter {
    argon: Argon2<'static>,
}

impl Argon2Encrypter {
    pub fn new() -> Self {
        Self {
            argon: Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default()),
        }
    }
}

impl Default for Argon2Encrypter {
    fn default() -> Self {
        Self::new()
    }
}

impl Encrypt for Argon2Encrypter {
    fn encrypt(&self, password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        self.argon
            .hash_password(password.as_bytes(), &salt)
            // TODO: remove unwrap later
            .unwrap()
            .to_string()
    }
}

impl Encrypter for Argon2Encrypter {
    fn verify(&self, hashed: &str, pwd: &str) -> bool {
        let Ok(hash) = PasswordHash::new(hashed) else {
            return false;
        };
        self.argon.verify_password(pwd.as_bytes(), &hash).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use domain::user::password::Password;

    use super::*;

    #[test]
    fn argon2_encryption_works() {
        let pwd = "Test";
        let encrypter = Argon2Encrypter::new();
        let encrypted_pwd = Password::new(pwd, &encrypter);
        assert!(encrypter.verify(encrypted_pwd.as_str(), pwd));
    }
}
