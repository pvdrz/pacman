extern crate pacman;

use pacman::agents::GreedyAgent;
use pacman::engine::Text;
use pacman::game::GameBuilder;

use pacman::game::Agent;
use pacman::game::Direction;
use pacman::game::GameState;
use pacman::util::manhattan_distance as distance;

struct ChaserGhost;

impl Agent for ChaserGhost {
    fn get_action(&mut self, state: &GameState, index: usize) -> Direction {
        let pacman_position = state.agent_position(0).unwrap();
        let actions = state.legal_actions(index);

        let mut best_distance = usize::max_value();
        let mut best_action = Direction::Stop;

        for action in actions {
            if let Ok(successor) = state.gen_successor(index, action) {
                let position = successor.agent_position(index).unwrap();
                let distance = distance(position, pacman_position);
                if distance <= best_distance {
                    best_distance = distance;
                    best_action = action;
                }
            }
        }
        best_action
    }
}

fn main() {
    let mut game = GameBuilder::new()
        .load_file("layouts/small_classic.lay")
        .engine(Text::new(400))
        .pacman(GreedyAgent)
        .ghost(ChaserGhost)
        .ghost(ChaserGhost)
        .finish()
        .unwrap();

    game.run().unwrap();
}
