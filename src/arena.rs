pub mod arena {

    use bmp::*;
    use nalgebra::*;

    pub struct Arena {
        pub width: usize,
        pub height: usize,
        pub floor: char,
        pub walls: char,
    }
    impl Arena {
        pub fn set_arena(arena: Arena) {
            let mut matrix = DMatrix::from_element(arena.height, arena.width, arena.floor);

            let map = bmp::open("img/0.bmp").unwrap();
            for (x, y) in map.coordinates() {
                if map.get_pixel(x, y) == Pixel::new(0, 0, 0) {
                    let x: usize = usize::try_from(x).unwrap();
                    let y: usize = usize::try_from(y).unwrap();
                    matrix[(x, y)] = '⧠';
                }
            }
            print!("{}", matrix);
        }
    }
}
