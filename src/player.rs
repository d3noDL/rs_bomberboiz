pub mod player {
    pub struct Player {
        pub position_x: usize,
        pub position_y: usize,
        pub character: String,
    }
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Player {
        pub fn move_to(&mut self, direction: Direction) {
            match direction {
                Direction::Up => self.position_y -= 1,
                Direction::Down => self.position_y += 1,
                Direction::Left => self.position_x -= 1,
                Direction::Right => self.position_x += 1,
            }
        }
        pub fn plant_bomb() {}
    }
}
