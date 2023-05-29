pub mod arena {
    use nalgebra::*;

    pub struct Arena {
        pub width: usize,
        pub height: usize,
        pub floor: char,
        pub walls: char,
    }
    impl Arena {
        pub fn draw_arena(arena: Arena) {
            let matrix = DMatrix::from_element(arena.height, arena.width, arena.floor);
            print!("{}", matrix);
        }
    }
}
