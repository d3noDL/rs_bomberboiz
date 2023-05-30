pub mod player {
    pub struct Player {
        pub name: String,
        pub position_x: usize,
        pub position_y: usize,
    }
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Player {
        pub fn move_in_direction(&mut self, direction: Direction) {
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
