use game::Agent;
use game::Direction;
use game::GameState;
use rand::{thread_rng, Rng};

pub struct RandomAgent {
    index: usize,
}

impl RandomAgent {
    pub fn new(index: usize) -> Self {
        RandomAgent { index }
    }
}

impl Agent for RandomAgent {
    fn get_action(&self, state: &GameState) -> Direction {
        let actions = state.legal_actions(self.index);
        let action = thread_rng()
            .choose(&actions)
            .cloned()
            .unwrap_or(Direction::Stop);
        action
    }

    fn get_index(&self) -> usize {
        self.index
    }
}

pub struct GreedyAgent {
    index: usize,
}

impl GreedyAgent {
    pub fn new(index: usize) -> Self {
        GreedyAgent { index }
    }
}

impl Agent for GreedyAgent {
    fn get_action(&self, state: &GameState) -> Direction {
        let mut actions = state.legal_actions(self.index);
        actions.remove_item(&Direction::Stop);
        let scores = actions
            .iter()
            .filter_map(|&action| {
                state
                    .gen_successor(self.index, action)
                    .map(|succ| (action, succ.score()))
                    .ok()
            })
            .collect::<Vec<_>>();
        if let Some(max_score) = scores.iter().map(|(_, s)| s).max() {
            let best_actions = scores
                .iter()
                .filter(|(_, s)| s == max_score)
                .map(|(a, _)| a.clone().clone())
                .collect::<Vec<Direction>>();
            thread_rng()
                .choose(&best_actions)
                .cloned()
                .unwrap_or(Direction::Stop)
        } else {
            Direction::Stop
        }
    }

    fn get_index(&self) -> usize {
        self.index
    }
}
