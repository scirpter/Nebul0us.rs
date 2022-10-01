#![allow(dead_code)]
#![allow(unused_imports)]

use std;

mod game;
mod models;

fn main() {
    let mut b_arr = models::ByteArray::new(None);

    b_arr.write_hex("760000000047599abb0073d43e00000077ff000000ffff0000d100ff00ff0000039183007777000000ffff78ffe600ff000000770000fbffff000000000000930900301b770066e100000500ff0004a30000d000ffa303010000a10000ed");

    let decrypted = game::netrng::decrypt(b_arr);

    println!("Decrypted: {}", decrypted.hex_data());

    let key = 0x9999999999999999;
    let encrypted = game::netrng::encrypt(decrypted, key);

    println!("Encrypted: {}", encrypted.hex_data());
}
