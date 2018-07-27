use game::Agent;
use game::Direction;
use game::GameState;

use rand::thread_rng;
use rand::Rng;
pub struct RandomAgent;

impl Agent for RandomAgent {
    fn get_action(&self, state: &GameState, index: usize) -> Direction {
        let actions = state.legal_actions(index);
        let action = thread_rng().choose(&actions).unwrap_or(&Direction::Stop);
        *action
    }
}
