use crate::net;
use java_rand::Random as JavaRNG;

#[cfg(test)]
/// Tests if the en-/decryption is working properly
fn test_encrypted_eq_decrypted() {
    let encrypted_hex = "760000000047599ABB0073D43E00000077FF000000FFFF0000D100FF00FF0000039183007777000000FFFF78FFE600FF000000770000FBFFFF000000000000930900301B770066E100000500FF0004A30000D000FFA303010000A10000ED";

    let mut encrypted_b_arr = net::ByteArray::new(None);
    encrypted_b_arr.write_hex(encrypted_hex);

    let decrypted = decrypt(encrypted_b_arr);

    let reencrypted = encrypt(decrypted, 0x47599ABB0073D43E);
    println!("Reencrypted: {}", reencrypted.hex_data());
    assert_eq!(reencrypted.hex_data(), encrypted_hex);
}

#[derive(Clone, Copy)]
struct K2 {
    xlength: usize,
    xrandint: i32,
}

impl K2 {
    fn new(xlength: usize, xrandint: i32) -> Self {
        K2 { xlength, xrandint }
    }
}

pub fn encrypt(mut b_arr: net::ByteArray, key: u64) -> net::ByteArray {
    let mut length = b_arr.data.len() - 13;
    let mut array_list: Vec<K2> = vec![];
    let mut random = JavaRNG::new(key);
    loop {
        length -= 1;
        if length <= 0 {
            break;
        }
        array_list.push(K2::new(length, random.next_i32_bound((length as i32) + 1)));
    }
    for i in 0..array_list.len() {
        let k2 = array_list[i];
        let i11 = k2.xlength + 13;
        let b10 = b_arr.data[i11];
        let i12 = k2.xrandint + 13;
        b_arr.data[i11] = b_arr.data[i12 as usize];
        b_arr.data[i12 as usize] = b10;
    }

    return b_arr;
}

pub fn decrypt(mut b_arr: net::ByteArray) -> net::ByteArray {
    let mut length = b_arr.data.len() - 13;
    let mut array_list: Vec<K2> = vec![];
    b_arr.read_byte(); // skip: packet id
    b_arr.read_int(); // skip: null int
    let mut random = JavaRNG::new(b_arr.read_long());
    loop {
        length -= 1;
        if length <= 0 {
            break;
        }
        array_list.push(K2::new(length, random.next_i32_bound((length as i32) + 1)));
    }
    let i10 = array_list.len() - 1;
    for i in (0..=i10).rev() {
        let k2 = array_list[i];
        let i11 = k2.xlength + 13;
        let b10 = b_arr.data[i11];
        let i12 = k2.xrandint + 13;
        b_arr.data[i11] = b_arr.data[i12 as usize];
        b_arr.data[i12 as usize] = b10;
    }
    return b_arr;
}
