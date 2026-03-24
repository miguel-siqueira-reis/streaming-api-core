use argon2::{
  password_hash::{
    rand_core::OsRng,
    PasswordHash, PasswordHasher, PasswordVerifier, SaltString
  },
  Argon2
};

use crate::features::auth::error::AuthError;

pub fn hash_password(password: &str) -> Result<String, AuthError> {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();

  let password_hash = argon2.hash_password(password.as_bytes(), &salt).map_err(|_| AuthError::PasswordHashingError)?.to_string();

  Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AuthError> {
  let parsed_hash = PasswordHash::new(password_hash).map_err(|_| AuthError::PasswordHashingError)?;
  let argon2 = Argon2::default();

  let compare_password = argon2.verify_password(password.as_bytes(), &parsed_hash);
  Ok(compare_password.is_ok())
}
