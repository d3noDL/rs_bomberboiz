extern crate bmp;
extern crate nalgebra;
extern crate text_io;

use nalgebra::*;
use rs_test::arena::arena::{Arena, Build, Update};
use rs_test::player::player::Player;
use text_io::*;

fn main() {
    clear();
    // let mut players: Vec<Player> = Vec::new();
    // clear();
    // print!("Enter the number of players: ");
    // let player_count: usize = read!();
    // clear();
    // for i in 0..player_count {
    //     print!("Enter the name for Player {}: ", i + 1);
    //     let p_name: String = read!();
    //     clear();

    //     players.push(Player {
    //         name: p_name,
    //         position_x: 0,
    //         position_y: 0,
    //     })
    // }
    clear();
    let arena_matrix = DMatrix::from_element(0, 0, "elem".to_string());
    let mut arena: Arena = Arena {
        matrix: arena_matrix,
    };
    arena.build();
    arena.update();
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
