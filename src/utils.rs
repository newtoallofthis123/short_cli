use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use regex::Regex;

pub fn is_valid_url(url: &str) -> bool {
    // Regex to check if the URL is valid
    //? Obviously this is not the best regex, but it works for now
    let url_regex = Regex::new(r#"^(https?://)?[a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,5}(:[0-9]{1,5})?(\/.*)?$"#).unwrap();
    url_regex.is_match(url)
}

pub fn exit(msg: &str){
    //* Peacefully exit the program with an error message */ 
    bunt::println!("{$red}{}{/$}, exiting program...", msg);
    print_footer();
    std::process::exit(1);
}

//* Print a small footer */
pub fn print_footer(){
    bunt::println!("Thanks for using {$blue}NoobShort{/$}, made with {$red}♥{/$} by {$yellow}Ishan Joshi{/$}");
}

//* Pretty print the result */
pub fn print_result(json: &serde_json::Value){
    let slug = json.get("slug").unwrap().as_str().unwrap();
    let url = format!("https://www.noobscience.rocks/go/{}", slug);
    bunt::println!("{$green}Success!{/$} Shortened URL: {$blue}{}{/$}", url);
    copy(&url);
    bunt::println!("{$blue}Copied to Clipboard!{/$}");
}

//* Copy the shortened URL to the clipboard */
pub fn copy(msg: &str){
    let mut board: ClipboardContext = ClipboardProvider::new().unwrap();
    board.set_contents(msg.to_owned()).unwrap();
}

//* Print a slash screen */
pub fn slash_screen(){

    let figure = r#"
    _   __            __   _____ __               __ 
    / | / /___  ____  / /_ / ___// /_  ____  _____/ /_
   /  |/ / __ \/ __ \/ __ \\__ \/ __ \/ __ \/ ___/ __/
  / /|  / /_/ / /_/ / /_/ /__/ / / / / /_/ / /  / /_  
 /_/ |_/\____/\____/_.___/____/_/ /_/\____/_/   \__/  
    "#;

    bunt::println!("{$blue}{}{/$}", figure);
    let line_break = "-".repeat(figure.to_string().lines().next().unwrap().len());
    bunt::println!("{$blue}{}{/$}", line_break);
}