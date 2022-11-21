#![allow(non_snake_case)]

use crate::{
    game::{enums, packets},
    models::client,
    net, print,
};

pub async fn CONNECT_REQUEST_3<'a>(bot: &mut client::Bot<'a>) {
    let packet = packets::ConnectRequest3::new(enums::GameMode::FFA_ULTRA, false, false);
    let data = packet.write(bot);

    let sock = bot.net.sock.as_mut().unwrap();
    sock.send_to(&data, (bot.net.server_ip.as_str(), bot.net.server_port))
        .await
        .unwrap();

    print::log(
        "Debug",
        &format!("{} --> {}", "CONNECT_REQUEST_3", bot.net.server_ip),
    );
}

pub async fn KEEP_ALIVE<'a>(bot: &mut client::Bot<'a>) {
    let packet = packets::KeepAlive::new();
    let data = packet.write(bot);

    let sock = bot.net.sock.as_mut().unwrap();
    sock.send_to(&data, (bot.net.server_ip.as_str(), bot.net.server_port))
        .await
        .unwrap();

    print::log(
        "Debug",
        &format!("{} --> {}", "KEEP_ALIVE", bot.net.server_ip),
    );
}

pub fn OBJ_DATA_RELATIVE(b_arr: &mut net::ByteArray, f: f32, f2: f32) -> f32 {
    return (((b_arr.read_byte() & 255) << 16)
        + ((b_arr.read_byte() & 255) << 8)
        + (b_arr.read_byte() & 255)) as f32
        * (f2 - f)
        / 1.6777215e7
        + f;
}

pub fn LE_SHORT_INT(b_arr: &mut net::ByteArray) -> u8 {
    ((b_arr.read_byte() & 255) << 16) + ((b_arr.read_byte() & 255) << 8) + (b_arr.read_byte() & 255)
}

pub fn OBJ_ANGLE(b_arr: &mut net::ByteArray, f: f32, f2: f32) -> f32 {
    return (((b_arr.read_byte() & 255) as f32) * (f2 - f)) / 255.0 + f;
}
