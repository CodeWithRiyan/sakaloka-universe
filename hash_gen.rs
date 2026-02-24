use sakaloka_secure::argon2::hash_password;
use sakaloka_secure::newtypes::Password;

fn main() {
    let pw = Password::new("SakalokaNeptune123!").unwrap();
    let hash = hash_password(&pw).unwrap();
    println!("{}", hash);
}
