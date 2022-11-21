use crate::game::enums;
use crate::game::funx::*;
use crate::models;
use crate::models::client;
use crate::net;

/// ## ?
/// Received to obtain raw
/// game object data like hole
/// positions, player splits, ...
pub struct GameUpdate {
    pub enum_type: enums::Packet,
    data: Vec<u8>,
}

impl GameUpdate {
    pub fn new(data: Vec<u8>) -> Self {
        GameUpdate {
            enum_type: enums::Packet::GAME_UPDATE,
            data,
        }
    }

    pub async fn parse<'b>(&mut self, bot: &mut client::Bot<'b>) {
        let data = self.data.clone();
        let mut b_arr = net::ByteArray::new(Some(data));

        let packet_id = b_arr.read_byte();
        b_arr.read_byte(); // 0xFF || 0x00 || 0x0F
        bot.net.world.tick = b_arr.read_byte();
        bot.net.world.raw_size = OBJ_DATA_RELATIVE(&mut b_arr, 0.0, 844.1067504882812); // 844.1067504882812 = size of type LARGE
        let unknown0 = b_arr.read_byte();
        let unknown1 = LE_SHORT_INT(&mut b_arr);
        let unknown2 = ((b_arr.read_byte() + 128) as u16) % 256;
        let unknown3 = ((b_arr.read_byte() + 128) as u16) % 256;
        let players_ct = b_arr.read_byte();
        let read_byte2 = b_arr.read_byte();
        let raw_event_ct = read_byte2 & 21;
        let paint_mode_related_event_ct = (read_byte2 & 224) >> 5;
        let ejection_event_ct = b_arr.read_byte();
        let dot_event_ct = ((b_arr.read_byte() + 128) as u16) % 256;
        let hole_event_ct = b_arr.read_byte();
        let ejection_move_ct = b_arr.read_byte();
        let read_byte3 = b_arr.read_byte();
        let item_event_ct = read_byte3 & 31;
        let unknown5 = (read_byte3 & 224) >> 5;
        let read_byte4 = b_arr.read_byte();
        let spell_event_ct = read_byte4 & 31;
        let wall_event_ct = (read_byte4 & 224) >> 5;
        let unknown6 = ((b_arr.read_byte() + 128) as u16) % 256;
        let unknown7 = b_arr.read_byte();

        if paint_mode_related_event_ct > 0 {
            let mut c = Vec::new();
            let mut d_x = Vec::new();
            let mut e_y = Vec::new();

            for id in 0..paint_mode_related_event_ct {
                c[id as usize] = b_arr.read_byte();
                d_x[id as usize] = OBJ_DATA_RELATIVE(&mut b_arr, 0.0, bot.net.world.raw_size);
                e_y[id as usize] = OBJ_DATA_RELATIVE(&mut b_arr, 0.0, bot.net.world.raw_size);
            }
        }

        if ejection_event_ct > 0 {
            let mut ej_id = Vec::new();
            let mut player_id = Vec::new();
            let mut ej_x = Vec::new();
            let mut ej_y = Vec::new();
            let mut ej_mass = Vec::new();
            let mut ej_angle = Vec::new();

            for id in 0..ejection_event_ct {
                let read_byte4 = b_arr.read_byte();
                let is_ej_destroyed = (read_byte4 & 1) == 1;
                ej_id[id as usize] = (read_byte4 & 254) >> 1;
                player_id[id as usize] = b_arr.read_byte();
                ej_x[id as usize] = OBJ_DATA_RELATIVE(&mut b_arr, 0.0, bot.net.world.raw_size);
                ej_y[id as usize] = OBJ_DATA_RELATIVE(&mut b_arr, 0.0, bot.net.world.raw_size);
                ej_angle[id as usize] = OBJ_ANGLE(&mut b_arr, 0.0, 6.2831855);

                if !is_ej_destroyed {
                    ej_mass[id as usize] = OBJ_DATA_RELATIVE(&mut b_arr, 0.0, 500000.0);
                    let ejection = models::Ejection {
                        id,
                        player_id: player_id[id as usize],
                        x: ej_x[id as usize],
                        y: ej_y[id as usize],
                        angle: ej_angle[id as usize],
                        mass: ej_mass[id as usize],
                    };
                    bot.net.world.ejections.insert(ej_id[id as usize], ejection);
                } else {
                    ej_mass[id as usize] = -std::f32::INFINITY;

                    if bot.net.world.ejections.contains_key(&id) {
                        bot.net.world.ejections.remove(&id);
                    }
                }
            }
        }

        // TODO: parse rest
    }
}
