extern crate pacman;

use pacman::agents::GreedyAgent;
use pacman::agents::RandomAgent;
use pacman::engine::Text;
use pacman::game::GameBuilder;

fn main() {
    let mut game = GameBuilder::new()
        .load_file("layouts/small_classic.lay")
        .engine(Text::new(400))
        .pacman(GreedyAgent::new(0))
        .ghost(RandomAgent::new(1))
        .ghost(RandomAgent::new(2))
        .finish()
        .unwrap();

    game.run().unwrap();
}
