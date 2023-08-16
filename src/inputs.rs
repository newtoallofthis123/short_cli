


use inquire::Text;

//* Uses inquire crate to get the URL from the user
pub fn get_url() -> String {
    let url = Text::new("Enter the URL: ").prompt().expect("Failed to get URL!, exiting...");
    return url;
}