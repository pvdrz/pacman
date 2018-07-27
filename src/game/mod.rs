mod agent;
mod agent_state;
mod configuration;
mod direction;
mod game_state;
mod grid;

pub use self::agent::Agent;
pub use self::agent_state::AgentState;
pub use self::configuration::Configuration;
pub use self::direction::Direction;
pub use self::game_state::GameState;
pub use self::grid::Grid;

use super::engine::Engine;

use failure::err_msg;
use failure::Error;

pub struct Game<'a, E: Engine> {
    agents: Vec<Box<Agent + 'a>>,
    state: GameState,
    engine: E,
}

impl<'a, E: Engine> Game<'a, E> {
    pub fn run(&mut self) -> Result<(), Error> {
        let mut index = 0;
        while !self.state.game_over() {
            if index == self.state.num_agents() {
                index = 0;
                self.engine.update(&self.state);
            }
            let agent = self.agents.get_mut(index).unwrap();
            let action = agent.get_action(&self.state, index);
            self.state = self.state.gen_successor(index, action)?;
            index += 1;
        }
        self.engine.update(&self.state);
        Ok(())
    }
}

pub struct GameBuilder<'a, E: Engine> {
    agents: Vec<Box<Agent + 'a>>,
    state: Option<GameState>,
    engine: Option<E>,
    has_pacman: bool,
}

impl<'a, E: Engine> GameBuilder<'a, E> {
    pub fn new() -> Self {
        GameBuilder {
            agents: Vec::new(),
            state: None,
            engine: None,
            has_pacman: false,
        }
    }

    pub fn pacman<A: Agent + 'a>(mut self, agent: A) -> Self {
        self.agents.insert(0, Box::new(agent));
        self.has_pacman = true;
        self
    }
    pub fn ghost<A: Agent + 'a>(mut self, agent: A) -> Self {
        self.agents.push(Box::new(agent));
        self
    }
    pub fn engine(mut self, engine: E) -> Self {
        self.engine = Some(engine);
        self
    }
    pub fn load_file(mut self, filename: &str) -> Self {
        self.state = GameState::from_file(filename).ok();
        self
    }
    pub fn finish(self) -> Result<Game<'a, E>, Error> {
        if true || self.has_pacman {
            Ok(Game {
                agents: self.agents,
                state: self.state.ok_or(err_msg("No level"))?,
                engine: self.engine.ok_or(err_msg("No engine"))?,
            })
        } else {
            Err(err_msg("No pacman agent"))
        }
    }
}
