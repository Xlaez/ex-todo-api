use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};


pub fn hash_password(pure_string: &str ) ->  Result<String, Error>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(pure_string.as_bytes(), &salt)?;
    Ok(hashed_password.to_string())
}

pub fn verify_password(hashed_string: &str, pure_string: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hashed_string)?;
    Ok(Argon2::default().verify_password(pure_string.as_bytes(), &parsed_hash).is_ok())
}