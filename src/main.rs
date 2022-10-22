#![allow(dead_code)]
#![allow(unused_variables)]

mod game;
mod interfaces;
mod models;
mod net;
mod utils;

use game::enums;
use game::packets;
use inquire::{required, CustomType, MultiSelect, Select, Text};
use models::BotFunx;
use utils::pretty_print as print;

#[tokio::main]
async fn main() {
    print::clear_console();

    let mut bots: Vec<models::Bot> = vec![];

    let bot_amount: u8 = CustomType::new("Bot amount? (max = 7)")
        .with_help_message("Maximum clients connected per IP is 7")
        .with_placeholder("default = 1")
        .with_default(1)
        .with_error_message("Please type a valid number")
        .prompt()
        .unwrap();
    let bot_name = Text::new("Bot name? (max 15 characters)")
        .with_help_message("The name the bots should connect with")
        .with_placeholder("default = nyaanity ")
        .with_default("nyaanity ")
        .prompt()
        .unwrap();
    let server: enums::Server = Select::new("Server?", enums::Server::iter())
        .with_help_message("The server the bots should connect to")
        .with_starting_cursor(enums::Server::EUROPE as usize)
        .prompt()
        .unwrap();

    // command exec loop
    loop {}
}
