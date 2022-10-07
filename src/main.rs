#![allow(dead_code)]
#![allow(unused_variables)]

use tokio;

mod game;
mod interfaces;
mod models;
mod net;
mod utils;

use game::enums;
use game::packets;
use std::net::UdpSocket;
use utils::pretty_print as print;

#[tokio::main]
async fn main() {
    print::clear_console();

    let bot = models::Bot::new("test", None);

    let cr_packet = packets::ConnectRequest3::new(&bot, enums::GameMode::FFA_ULTRA, false, false);

    let server_ip = "172.105.248.252:27901";

    // bind to random open port in local machine

    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");

    // send packet to server

    socket
        .send_to(&cr_packet.write(), server_ip)
        .expect("couldn't send packet");

    print::log("NETLOG", "Sent packet to server");

    loop {}
}
