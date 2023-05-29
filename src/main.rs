extern crate bmp;
extern crate text_io;
use rs_test::arena::arena::Arena;
use rs_test::player::player::Player;
use text_io::*;

fn main() {
    let mut players: Vec<Player> = Vec::new();

    print!("Enter the number of players: ");
    let player_count: usize = read!();
    for i in 0..player_count {
        println!("Enter the name for Player {}", i + 1);
        let p_name: String = read!();
        println!("Pick a character for {}", p_name);
        let p_char: char = read!();

        players.push(Player {
            name: p_name,
            character: p_char,
            position_x: 0,
            position_y: 0,
        })
    }

    let arena: Arena = Arena {
        width: 15,
        height: 15,
        floor: ' ',
        walls: ' ',
    };

    Arena::set_arena(arena);

    update();
    draw();
}

fn update() {
    loop {
        break;
    }
}

fn draw() {
    loop {
        break;
    }
}
