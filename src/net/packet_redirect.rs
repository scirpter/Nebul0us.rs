use crate::game::packets;
use crate::models::client;
use crate::print;

pub async fn recv_n_redirect<'b>(bot: &mut client::Bot<'b>) {
    let mut buf = [0; 1024];
    let result = bot.net.sock.as_mut().unwrap().try_recv_from(&mut buf);

    match result {
        Ok((size, addr)) => {
            let data = &buf[..size];
            let packet_id = data[0];

            match packet_id {
                0x01 => {
                    let mut packet = packets::ConnectResult2::new(data.to_vec());
                    packet.parse(bot).await;
                }
                0x16 => {
                    let mut packet = packets::GameUpdate::new(data.to_vec());
                    packet.parse(bot).await;
                }
                _ => print::wtf("ID Not Found", &format!("0x{:0X}", packet_id)),
            }
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::WouldBlock {
                // no data available
            } else {
                print::error("Error", &format!("{:?}", e));
            }
        }
    }
}
