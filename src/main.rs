#![allow(dead_code)]
#![allow(unused_variables)]

extern crate tokio;

mod game;
mod interfaces;
mod models;

use game::packets;
use models::pretty_print as print;

#[tokio::main]
async fn main() {
    print::clear_console();

    loop {}
}
