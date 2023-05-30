pub mod arena {

    use bmp::*;

    use colored::Colorize;
    use nalgebra::*;

    pub fn build_arena() -> Matrix<String, Dyn, Dyn, VecStorage<String, Dyn, Dyn>> {
        let map = bmp::open("img/0.bmp").unwrap();
        let mut matrix = DMatrix::from_element(15, 15, " ".to_string());

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
        return matrix;
    }
    pub fn update_arena(arena: Matrix<String, Dyn, Dyn, VecStorage<String, Dyn, Dyn>>) {
        let mut pos = 0;
        for i in arena.iter() {
            if pos < 15 - 1 {
                print!("{} ", i);
                pos += 1;
            } else {
                pos = 0;
                println!("{} ", i);
            }
        }
    }
}
