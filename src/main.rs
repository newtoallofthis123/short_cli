use clap::Parser;
use human_panic::setup_panic;

#[derive(Parser, Debug)]
#[command(name="NoobShort", author="Ishan Joshi", version, about="A Simple CLI for NoobShort", long_about = None)]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=false)]
    url: Option<String>,

    #[arg(short, long)]
    custom: Option<String>,

    #[arg(short, long)]
    docs: bool,
}

// Include all the mods
mod inputs;
mod utils;
mod web;

fn get_inputs()-> Args{
    let args = Args::parse();
    return args;
}

#[tokio::main]
async fn main() {
    setup_panic!();
    utils::slash_screen();
    let args = get_inputs();

    if args.docs {
        utils::print_footer();
        bunt::println!("You can find the docs at {$blue}https://github.com/newtoallofthis123/short_cli{/$}");
        utils::copy("https://github.com/newtoallofthis123/short_cli");
        bunt::println!("It is copied to your clipboard!");
        std::process::exit(0);
    }
    let url = args.url.unwrap_or_else(|| inputs::get_url());

    if utils::is_valid_url(&url) {
        bunt::println!("{$green}Valid{/$} URL, continuing...");
    } else {
        utils::exit("Invalid URL!");
    }
    let res: serde_json::Value;
    if args.custom.is_some() {
        bunt::println!("{$green}Custom{/$} slug provided, continuing...");
        let password = inputs::get_password();
        let password_hash = inputs::hash_password(&password);
        if password_hash != "493196b35d0d79e6c920dd6033d14dc6b0a22731b165c9e61eb517fc2da46d97"{
            utils::exit("Invalid password!");
        }
        res = web::send_request(&url, &args.custom.unwrap(), true).await;
    } else {
        res = web::send_request(&url, &"", false).await;
    }
    utils::print_result(&res);
    utils::print_footer();
}

