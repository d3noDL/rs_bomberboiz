extern crate bmp;
extern crate nalgebra;
extern crate text_io;

use colored::Colorize;
use nalgebra::*;
use rs_test::arena::arena::Arena;
use rs_test::player::player::Player;
use std::io::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
//use text_io::*;

fn main() {
    clear();

    let arena_matrix = DMatrix::from_element(0, 0, "elem".to_string());
    let player_name = String::from("d3noDL");
    let mut arena: Arena = Arena {
        matrix: arena_matrix,
    };
    let mut player: Player = Player {
        name: player_name,
        position_x: 1,
        position_y: 1,
    };

    arena.build();
    arena.matrix[(player.position_x, player.position_y)] = "o".green().to_string();
    arena.update();

    let input = stdin();

    for c in input.keys() {
        //i reckon this speaks for itself
        match c.unwrap() {
            Key::Ctrl('h') => println!("Hello world!"),
            Key::Ctrl('q') => break,
            Key::Alt('t') => println!("termion is cool"),
            _ => (),
        }
    }
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
