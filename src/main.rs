extern crate bmp;
extern crate text_io;

use rs_test::arena::arena::Arena;
use rs_test::player::player::Player;
use text_io::*;

fn main() {
    clear();
    let mut players: Vec<Player> = Vec::new();
    let arena: Arena = Arena {
        width: 15,
        height: 15,
        floor: " ".to_string(),
        walls: " ".to_string(),
    };
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
    Arena::set_arena(arena);

    'update: loop {
        break 'update;
    }

    'draw: loop {
        break 'draw;
    }
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
