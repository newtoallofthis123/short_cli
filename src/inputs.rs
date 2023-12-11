use inquire::{Text, Password, PasswordDisplayMode};
use sha2::{Sha256, Digest};

//* Uses inquire crate to get the URL from the user
pub fn get_url() -> String {
    Text::new("Enter the URL: ").prompt().expect("Failed to get URL!, exiting...")
}

pub fn get_password() -> String{
    Password::new("Enter a sudo password:")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt().unwrap()
}   

/// Okay, so the way I am planning to handle this password is not the most secure one.
/// Point being, I don't care if you know the password for custom hashes, since it is not
/// a big deal. I am not storing any of the passwords.
/// So, I am using a simple SHA256 hash to hash the password, and then I am using that hash
/// as the password. I know, it is not the best way to do it, but it works.
/// And hey! If you want to make it more secure, you can always make a PR!

pub fn hash_password(password: &str) -> String{
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();
    format!("{:x}", result)
}