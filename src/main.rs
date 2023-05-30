extern crate bmp;
extern crate nalgebra;
extern crate text_io;

use colored::Colorize;
use device_query::DeviceQuery;
use device_query::DeviceState;
use device_query::Keycode;
use nalgebra::*;
use rs_test::arena::arena::Arena;
use rs_test::player::player::Player;
//use text_io::*;

fn main() {
    clear();

    let arena_matrix = DMatrix::from_element(0, 0, "elem".to_string());
    let player_name = String::from("d3noDL");
    let mut arena: Arena = Arena {
        matrix: arena_matrix,
    };
    let player: Player = Player {
        name: player_name,
        position_x: 1,
        position_y: 1,
    };

    arena.build();
    arena.matrix[(player.position_x, player.position_y)] = "o".green().to_string();
    arena.update();

    let device_state = DeviceState::new();
    'update: loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            if key == &Keycode::Escape {
                println!("hi");
                break 'update;
            }
        }
    }
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
