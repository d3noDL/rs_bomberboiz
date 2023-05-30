extern crate bmp;
extern crate text_io;

use rs_test::arena::*;
use rs_test::player::player::Player;
use text_io::*;

fn main() {
    clear();
    let mut players: Vec<Player> = Vec::new();
    clear();
    print!("Enter the number of players: ");
    let player_count: usize = read!();
    clear();
    for i in 0..player_count {
        print!("Enter the name for Player {}: ", i + 1);
        let p_name: String = read!();
        clear();

        players.push(Player {
            name: p_name,
            position_x: 0,
            position_y: 0,
        })
    }
    clear();
    let arena_matrix = arena::build_arena();

    'update: loop {
        break 'update;
    }

    'draw: loop {
        arena::update_arena(arena_matrix);
        break 'draw;
    }
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
