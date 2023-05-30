pub mod arena {
    use bmp::*;
    use colored::Colorize;
    use nalgebra::*;

    pub struct Arena {
        pub matrix: Matrix<String, Dyn, Dyn, VecStorage<String, Dyn, Dyn>>,
    }

    impl Arena {
        pub fn build(&mut self) {
            let map = bmp::open("img/0.bmp").unwrap();
            self.matrix = DMatrix::from_element(15, 15, " ".to_string());

            for (x, y) in map.coordinates() {
                if map.get_pixel(x, y) == Pixel::new(0, 0, 0) {
                    let x: usize = usize::try_from(x).unwrap();
                    let y: usize = usize::try_from(y).unwrap();
                    self.matrix[(x, y)] = "▢".to_string();
                }
                if map.get_pixel(x, y) == Pixel::new(255, 255, 255) {
                    if rand::random() {
                        let x: usize = usize::try_from(x).unwrap();
                        let y: usize = usize::try_from(y).unwrap();
                        self.matrix[(x, y)] = "⊞".truecolor(255, 125, 0).to_string();
                    }
                }
            }
        }
        pub fn update(&self) {
            let mut pos = 0;
            for i in self.matrix.iter() {
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
}
