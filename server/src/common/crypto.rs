use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};

pub fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    // Hash password to PHC string ($pbkdf2-sha256$...)
    Pbkdf2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn password_authenticate(provided: String, actual: String) -> bool {
    let pw_hash = PasswordHash::new(&provided).unwrap();
    match Pbkdf2.verify_password(actual.as_bytes(), &pw_hash) {
        Ok(_v) => {
            true
        }
        Err(_e) => {
            false
        }
    }
}