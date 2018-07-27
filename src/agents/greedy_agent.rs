use game::Agent;
use game::Direction;
use game::GameState;

use rand::thread_rng;
use rand::Rng;

pub struct GreedyAgent;

impl Agent for GreedyAgent {
    fn get_action(&mut self, state: &GameState, index: usize) -> Direction {
        let mut actions = state.legal_actions(index);
        actions.remove_item(&Direction::Stop);

        let mut best_score = isize::min_value();
        let mut best_actions = vec![Direction::Stop];

        for action in actions {
            if let Ok(score) = state.gen_successor(index, action).map(|s| s.score()) {
                if score > best_score {
                    best_score = score;
                    best_actions.clear();
                    best_actions.push(action);
                } else if score == best_score {
                    best_actions.push(action);
                }
            }
        }

        *thread_rng()
            .choose(&best_actions)
            .unwrap_or(&Direction::Stop)
    }
}
