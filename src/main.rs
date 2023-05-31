extern crate bmp;
extern crate nalgebra;
extern crate text_io;

use colored::Colorize;
use console::*;
use nalgebra::*;
use rs_test::arena::arena::Arena;
use rs_test::player::player::Player;

fn main() {
    let term = Term::stdout();

    term.clear_screen().expect("Couldn't clear screen");

    let arena_matrix = DMatrix::from_element(0, 0, "elem".to_string());
    let mut arena: Arena = Arena {
        matrix: arena_matrix,
        floor: " ".to_string(),
        wall: "▢".to_string(),
        breakable: "⊞".to_string(),
    };
    let mut player_1: Player = Player {
        position_x: 1,
        position_y: 1,
        character: "o".green().to_string(),
    };

    let mut player_2: Player = Player {
        position_x: 13,
        position_y: 13,
        character: "o".red().to_string(),
    };

    arena.build();
    arena.matrix[(player_1.position_x, player_1.position_y)] = player_1.character.to_string();
    arena.matrix[(player_2.position_x, player_2.position_y)] = player_2.character.to_string();
    arena.update();

    'update: loop {
        let _input = match term.read_char().expect("Couldn't read char") {
            'w' => move_player(&mut player_1, &mut arena, 'w'),
            'a' => move_player(&mut player_1, &mut arena, 'a'),
            's' => move_player(&mut player_1, &mut arena, 's'),
            'd' => move_player(&mut player_1, &mut arena, 'd'),
            'i' => move_player(&mut player_2, &mut arena, 'i'),
            'j' => move_player(&mut player_2, &mut arena, 'j'),
            'k' => move_player(&mut player_2, &mut arena, 'k'),
            'l' => move_player(&mut player_2, &mut arena, 'l'),

            'q' => {
                term.clear_screen().expect("Couldn't clear screen");
                break 'update;
            }
            _ => {}
        };
    }
}

fn move_player(player: &mut Player, arena: &mut Arena, dir: char) {
    let last_pos_x = player.position_x;
    let last_pos_y = player.position_y;
    match dir {
        'w' | 'i' => {
            if arena.matrix[(player.position_y - 1, player.position_x)] != arena.wall.to_string()
                && arena.matrix[(player.position_y - 1, player.position_x)]
                    != arena.breakable.truecolor(255, 125, 0).to_string()
            {
                player.position_y -= 1;
            }
        }
        'a' | 'j' => {
            if arena.matrix[(player.position_y, player.position_x - 1)] != arena.wall.to_string()
                && arena.matrix[(player.position_y, player.position_x - 1)]
                    != arena.breakable.truecolor(255, 125, 0).to_string()
            {
                player.position_x -= 1;
            }
        }
        's' | 'k' => {
            if arena.matrix[(player.position_y + 1, player.position_x)] != arena.wall.to_string()
                && arena.matrix[(player.position_y + 1, player.position_x)]
                    != arena.breakable.truecolor(255, 125, 0).to_string()
            {
                player.position_y += 1;
            }
        }
        'd' | 'l' => {
            if arena.matrix[(player.position_y, player.position_x + 1)] != arena.wall.to_string()
                && arena.matrix[(player.position_y, player.position_x + 1)]
                    != arena.breakable.truecolor(255, 125, 0).to_string()
            {
                player.position_x += 1;
            }
        }
        _ => {}
    }
    arena.matrix[(last_pos_x, last_pos_y)] = " ".to_string();
    arena.matrix[(player.position_x, player.position_y)] = player.character.to_string();
    let term = Term::stdout();
    term.clear_screen().expect("Can't clear screen");
    arena.update();
}
