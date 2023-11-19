use crate::constants;
use crate::game::types::{NestedSimulation, Simulation};
use crate::handler::GameHandler;
use std::thread;
use std::time::Duration;

impl GameHandler {
    pub fn simulate_next_move(&mut self, positions: &Vec<(i8, i8)>) -> usize {
        let mut vec_simulation: Vec<Simulation> = Vec::new();
        for i in 0..positions.len() {
            let mut simulation_t0 = Simulation::new(positions[i]);
            self.table[positions[i].0 as usize][positions[i].1 as usize] = 1;
            let pos_first_complexity = self.get_positions_to_test();
            for k in 0..pos_first_complexity.len() {
                // println!("---------------SECOND LOOP----------");
                self.table[pos_first_complexity[k].0 as usize]
                    [pos_first_complexity[k].1 as usize] = 2;
                let mut simulation_t1 = NestedSimulation::new(pos_first_complexity[k]);
                self.simulate_games(&mut simulation_t1, &positions[i]);
                simulation_t1.calculate_percentages();
                simulation_t0.nested.push(simulation_t1);
                self.table[pos_first_complexity[k].0 as usize]
                    [pos_first_complexity[k].1 as usize] = 0;
            }
            self.combine_results(&mut simulation_t0);
            simulation_t0.self_simulation.calculate_percentages();
            vec_simulation.push(simulation_t0);
            self.table[positions[i].0 as usize][positions[i].1 as usize] = 0;
            // println!("---------------FIRST LOOP----------");
        }
        // println!("---------------FINAL----------");
        let index = self.analyze_best_move(&vec_simulation);
        index
    }

    fn simulate_games(&mut self, simulation_t1: &mut NestedSimulation, ai_pos: &(i8, i8)) {
        for _ in 0..constants::SIMULATIONS_AMOUNT {
            let index = self.simulate_random_game(ai_pos, &simulation_t1.next_move, true);
            simulation_t1.add_game(index);
        }
    }

    fn combine_results(&mut self, simulation_t0: &mut Simulation) {
        for i in 0..simulation_t0.nested.len() {
            simulation_t0.self_simulation.games.0 += simulation_t0.nested[i].games.0;
            simulation_t0.self_simulation.games.1 += simulation_t0.nested[i].games.1;
            simulation_t0.self_simulation.games.2 += simulation_t0.nested[i].games.2;
        }
    }
}
