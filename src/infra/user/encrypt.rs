use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordVerifier, Version};

use domain::user::password::Encrypt;

#[derive(Clone, Debug)]
pub struct Argon2Encrypt {
    argon: Argon2<'static>,
}

impl Argon2Encrypt {
    pub fn new() -> Self {
        Self {
            argon: Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default()),
        }
    }
}

impl Default for Argon2Encrypt {
    fn default() -> Self {
        Self::new()
    }
}

impl Encrypt for Argon2Encrypt {
    fn encrypt(&self, password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        self.argon
            .hash_password(password.as_bytes(), &salt)
            // TODO: remove unwrap later
            .unwrap()
            .to_string()
    }

    fn verify(&self, hashed: &str, pwd: &str) -> bool {
        let Ok(hash) = PasswordHash::new(hashed) else {
            return false;
        };
        self.argon.verify_password(pwd.as_bytes(), &hash).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use domain::user::password::PasswordEncrypter;

    use super::*;

    #[test]
    fn argon2_encryption_works() {
        let encrypter = PasswordEncrypter::new(Argon2Encrypt::new());
        let pwd = "Test";
        let encrypted_pwd = encrypter.encrypt(pwd);
        assert!(encrypter.verify(encrypted_pwd.as_str(), pwd));
    }
}
