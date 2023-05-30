pub mod arena {

    use bmp::*;

    use colored::Colorize;
    use nalgebra::*;

    pub struct Arena {
        pub width: usize,
        pub height: usize,
        pub floor: String,
        pub walls: String,
    }
    impl Arena {
        pub fn set_arena(arena: Arena) {
            let mut matrix = DMatrix::from_element(arena.height, arena.width, arena.floor);

            let map = bmp::open("img/0.bmp").unwrap();
            for (x, y) in map.coordinates() {
                if map.get_pixel(x, y) == Pixel::new(0, 0, 0) {
                    let x: usize = usize::try_from(x).unwrap();
                    let y: usize = usize::try_from(y).unwrap();
                    matrix[(x, y)] = "▢".to_string();
                }
                if map.get_pixel(x, y) == Pixel::new(255, 255, 255) {
                    if rand::random() {
                        let x: usize = usize::try_from(x).unwrap();
                        let y: usize = usize::try_from(y).unwrap();
                        matrix[(x, y)] = "⊞".truecolor(255, 125, 0).to_string();
                    }
                }
            }

            let mut pos = 0;
            for i in matrix.iter() {
                if pos < map.get_width() - 1 {
                    print!("{} ", i);
                    pos += 1;
                } else {
                    pos = 0;
                    println!("{} ", i);
                }
            }
        }
    }
}
