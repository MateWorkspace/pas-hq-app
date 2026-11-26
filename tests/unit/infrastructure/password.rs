use pas_hq_app::{
    domain::{contracts::utility::password::Password, models::PasswordError},
    infrastructure::utility::password::BcryptPassword,
};

fn password() -> BcryptPassword {
    // The minimum cost keeps the black-box tests fast.
    BcryptPassword::new(4)
}

#[test]
fn hashes_verify_and_mismatches_are_classified() {
    let password = password();
    let hash = password.hash("correct horse battery staple").unwrap();

    assert_ne!(hash, "correct horse battery staple");
    assert_eq!(
        password.compare(&hash, "correct horse battery staple"),
        Ok(())
    );
    assert_eq!(
        password.compare(&hash, "incorrect"),
        Err(PasswordError::Mismatch)
    );
}

#[test]
fn hashes_are_salted_per_call() {
    let password = password();
    let first = password.hash("secret").unwrap();
    let second = password.hash("secret").unwrap();

    assert_ne!(first, second);
    assert_eq!(password.compare(&first, "secret"), Ok(()));
    assert_eq!(password.compare(&second, "secret"), Ok(()));
}

#[test]
fn malformed_stored_hash_is_a_compare_failure() {
    let password = password();

    assert_eq!(
        password.compare("not-a-bcrypt-hash", "secret"),
        Err(PasswordError::CompareFailed)
    );
}

#[test]
fn passwords_beyond_the_bcrypt_boundary_are_rejected_without_truncation() {
    let password = password();
    let too_long = "x".repeat(73);
    let boundary = "x".repeat(72);

    assert_eq!(password.hash(&too_long), Err(PasswordError::HashFailed));

    let hash = password.hash(&boundary).unwrap();
    assert_eq!(password.compare(&hash, &boundary), Ok(()));
    assert_eq!(
        password.compare(&hash, &too_long),
        Err(PasswordError::CompareFailed)
    );
}

#[test]
fn out_of_range_cost_falls_back_to_the_default() {
    let low = BcryptPassword::new(1);
    let high = BcryptPassword::new(99);

    let low_hash = low.hash("secret").unwrap();
    assert_eq!(low.compare(&low_hash, "secret"), Ok(()));

    let high_hash = high.hash("secret").unwrap();
    assert_eq!(high.compare(&high_hash, "secret"), Ok(()));
}
