use game::Agent;
use game::Direction;
use game::GameState;

use rand::thread_rng;
use rand::Rng;

pub struct GreedyAgent;

impl Agent for GreedyAgent {
    fn get_action(&self, state: &GameState, index: usize) -> Direction {
        let mut actions = state.legal_actions(index);
        actions.remove_item(&Direction::Stop);
        thread_rng().shuffle(&mut actions);

        let mut best_score = isize::min_value();
        let mut best_action = Direction::Stop;

        for action in actions {
            if let Ok(score) = state.gen_successor(index, action).map(|s| s.score()) {
                if score >= best_score {
                    best_score = score;
                    best_action = action;
                }
            }
        }
        best_action
    }
}
