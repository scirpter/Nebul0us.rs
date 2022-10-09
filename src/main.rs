#![allow(dead_code)]
#![allow(unused_variables)]

mod game;
mod interfaces;
mod models;
mod net;
mod utils;

use game::enums;
use game::packets;
use models::UDPClient;
use utils::pretty_print as print;

#[tokio::main]
async fn main() {
    print::clear_console();

    let mut bot = models::Bot::new("test", None, ",-");

    bot.setup_sock().await;

    print::log("INIT", "Bot initialized.");

    loop {
        bot.udp_tick().await;
    }
}
