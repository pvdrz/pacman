use super::super::game::GameState;
use super::Engine;

use std::thread;
use std::time::Duration;

pub struct Text {
    delay: u64,
}

impl Text {
    pub fn new(delay: u64) -> Self {
        Text { delay }
    }
}

impl Engine for Text {
    fn update(&mut self, state: &GameState) {
        let (width, height) = state.walls().shape();

        let capsules = state.capsules();
        let pacman = state.agent_state(0).unwrap().config().position();
        let ghosts = (1..state.num_agents())
            .map(|index| state.agent_state(index).unwrap().config().position())
            .collect::<Vec<_>>();

        let mut buffer = format!("\nScore: {}\n", state.score());

        for y in 0..height {
            let mut line = Vec::new();
            for x in 0..width {
                if ghosts.contains(&(x, y)) {
                    line.push(b'G');
                } else if pacman == (x, y) {
                    line.push(b'P');
                } else if state.has_food(x, y) {
                    line.push(b'.');
                } else if state.has_wall(x, y) {
                    line.push(b'%')
                } else if capsules.contains(&(x, y)) {
                    line.push(b'o');
                } else {
                    line.push(b' ');
                }
            }
            buffer += &String::from_utf8(line).unwrap();
            buffer += "\n";
        }
        println!("{}", buffer);
        thread::sleep(Duration::from_millis(self.delay));
    }
}
