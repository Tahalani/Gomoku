use crate::handler::GameHandler;

impl GameHandler {
    pub fn get_next_move(&mut self) -> (i8, i8) {
        if self.turns == 0 {
            return self.get_first_move();
        }
        let mut positions = self.get_positions_to_test();
        let index = self.simulate_next_move(&mut positions);
        positions[index]
    }

    fn get_first_move(&self) -> (i8, i8) {
        (self.size.0 / 2, self.size.1 / 2)
    }
}
