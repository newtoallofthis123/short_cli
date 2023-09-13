use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

//? The URL is the URL of the API endpoint
//? You are free to use your own API endpoint, but make sure it has the same functionality
//? The underlying API returns the following JSON:
//? {
//?     "go_data": "mongoDB document of the shortened URL",
//?     "slug": "slug of the shortened URL",
//? }
const REQUEST_URL: &str = "https://www.noobscience.rocks/api/go";
const SUDO_REQUEST_URL: &str = "https://www.noobscience.rocks/api/go/sudo";

fn get_client()-> reqwest::Client{
    let client = reqwest::Client::new();
    return client;   
}

//* Simple Function to convert the response text to JSON */
fn convert_to_json(txt: &str)-> serde_json::Value{
    //Use serde_json to convert the response to JSON
    let json = serde_json::from_str(txt).unwrap();
    return json;
}

//* Uses Rust Async function powered by Tokio and reqwest to send the request to the API
//* Returns the JSON response
//* We use a indicatif progress bar to show the progress of the request
//* We send form data to the API, which is a HashMap
pub async fn send_request(url: &str, slug: &str, custom: bool)-> serde_json::Value{
    // Create a new reqwest client
    let client = get_client();

    // Create a HashMap to store the form data
    let mut form_data = HashMap::new();
    form_data.insert("url", url);
    form_data.insert("slug", slug);

    // Create a new progress bar
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(std::time::Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "⣾",
                "⣽",
                "⣻",
                "⢿",
                "⡿",
                "⣟",
                "⣯",
                "⣷",
                "(200 Ok)"
            ]),
    );
    pb.set_message("Sending Request...");

    // Send the request to the API
    let response = client.post(
        if custom {
            SUDO_REQUEST_URL
        } else {
            REQUEST_URL
        }
    )
        .form(&form_data)
        .send()
        .await
        .unwrap();

    // Get the response text
    let response_text = response.text().await.unwrap();

    // Stop the progress bar
    pb.finish_with_message("Acknowledged!");

    // Convert the response text to JSON
    let json = convert_to_json(&response_text);
    return json;    
}