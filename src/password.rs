use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("password hash should succeed")
        .to_string()
}

pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash =
        PasswordHash::new(&password_hash).expect("parsing password hash should succeed");
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}
