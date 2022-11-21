#![allow(dead_code)]
#![allow(unused_variables)]

mod game;
mod interfaces;
mod models;
mod net;
mod utils;

use crossbeam_channel::bounded;
use game::enums;
use inquire::{CustomType, Select, Text};
use models::client::{self, BotFunx};
use utils::pretty_print as print;

#[tokio::main]
async fn main() {
    print::clear_console();

    // communicate with all bots
    let mut senders: Vec<crossbeam_channel::Sender<client::Instruction>> = Vec::new();

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

    for i in 0..bot_amount {
        let (sender, receiver) = bounded(1);
        senders.push(sender);

        let mut bot = client::Bot::new(i, receiver, bot_name.clone(), None, server.ip().to_owned());

        tokio::spawn(async move {
            bot.cheat_loop().await;
        });
    }

    // command exec loop
    loop {
        let command = Text::new("Command?").prompt().unwrap();

        match command.as_str() {
            /*
            TODO: implement command handler to not spam the main file with commands inside ./src/models/bot/user_command_handler.rs
            */
            "exit" => {
                for sender in &senders {
                    sender.send(client::Instruction::new("exit", None)).unwrap();
                }
            }
            _ => {
                print::error("Error", "Command not found");
            }
        }
    }
}
