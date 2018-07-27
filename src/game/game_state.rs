use game::AgentState;
use game::Configuration;
use game::Direction;
use game::Grid;

use rules;

use failure::{err_msg, Error};
use std::fs::File;
use std::io::prelude::Read;

pub struct GameState {
    walls: Grid,
    food: Grid,
    capsules: Vec<(usize, usize)>,
    agent_states: Vec<AgentState>,
    score: isize,
    win: bool,
    lose: bool,
}

impl Clone for GameState {
    fn clone(&self) -> Self {
        GameState {
            walls: self.walls.clone(),
            food: self.food.clone(),
            capsules: self.capsules.clone(),
            agent_states: self.agent_states.clone(),
            score: self.score,
            win: self.win,
            lose: self.lose,
        }
    }
}

impl GameState {
    pub fn from_file(filename: &str) -> Result<Self, Error> {
        let mut buffer = String::new();
        let mut file = File::open(filename)?;
        file.read_to_string(&mut buffer)?;

        let width = buffer.chars().take_while(|&c| c != '\n').count();
        let height = buffer.chars().filter(|&c| c == '\n').count();

        let mut walls = Grid::new(width, height);
        let mut food = Grid::new(width, height);
        let mut capsules: Vec<(usize, usize)> = Vec::new();
        let mut agent_states: Vec<AgentState> = Vec::new();

        for (y, line) in buffer.lines().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                match chr {
                    '%' => walls.insert(x, y, true),
                    '.' => food.insert(x, y, true),
                    'o' => capsules.push((x, y)),
                    'P' => {
                        let start = Configuration::new(x, y, Direction::Stop);
                        agent_states.insert(0, AgentState::new(start, true));
                    }
                    'G' => {
                        let start = Configuration::new(x, y, Direction::Stop);
                        agent_states.push(AgentState::new(start, false));
                    }
                    ' ' => (),
                    '\n' => (),
                    _ => {
                        println!("Invalid character: {}", chr);
                        return Err(err_msg("Invalid character"));
                    }
                }
            }
        }
        Ok(GameState {
            walls,
            food,
            capsules,
            agent_states,
            score: 0,
            win: false,
            lose: false,
        })
    }

    pub fn legal_actions(&self, index: usize) -> Vec<Direction> {
        if index == 0 {
            rules::pacman::legal_actions(&self)
        } else if index < self.num_agents() {
            rules::ghost::legal_actions(&self, index)
        } else {
            Vec::new()
        }
    }

    pub fn gen_successor(&self, index: usize, action: Direction) -> Result<Self, Error> {
        if self.win || self.lose {
            Err(err_msg("No succesor for endgame"))
        } else {
            let mut state: GameState = self.clone();
            if index == 0 {
                rules::pacman::apply_action(&mut state, action);
                state.score += rules::ghost::check_death(&mut state, index);
                state.score -= 1;
                let (x, y) = state
                    .agent_state(0)
                    .ok_or(err_msg("Invalid index"))?
                    .config()
                    .position();
                if state.has_food(x, y) {
                    state.food.set(x, y, false);
                    state.score += 10;
                    if self.num_food() == 0 && !self.lose {
                        state.score += 500;
                        state.win = true;
                    }
                }
                if state.capsules().contains(&(x, y)) {
                    state.capsules.remove_item(&(x, y));
                    for ghost_index in 1..state.num_agents() {
                        state.agent_state_mut(ghost_index).unwrap().scare(40);
                    }
                }
            } else {
                rules::ghost::apply_action(&mut state, action, index);
                state.score += rules::ghost::check_death(&mut state, index);
            }
            Ok(state)
        }
    }

    pub fn agent_state(&self, index: usize) -> Option<&AgentState> {
        self.agent_states.get(index)
    }

    pub fn agent_state_mut(&mut self, index: usize) -> Option<&mut AgentState> {
        self.agent_states.get_mut(index)
    }

    pub fn agent_position(&self, index: usize) -> Option<(usize, usize)> {
        self.agent_states
            .get(index)
            .map(|agent| agent.config().position())
    }

    pub fn num_agents(&self) -> usize {
        self.agent_states.len()
    }

    pub fn score(&self) -> isize {
        self.score
    }

    pub fn capsules(&self) -> &Vec<(usize, usize)> {
        &self.capsules
    }

    pub fn num_food(&self) -> usize {
        self.food.count()
    }

    pub fn food(&self) -> &Grid {
        &self.food
    }

    pub fn walls(&self) -> &Grid {
        &self.walls
    }

    pub fn has_food(&self, x: usize, y: usize) -> bool {
        self.food.get(x, y).unwrap_or(false)
    }

    pub fn has_wall(&self, x: usize, y: usize) -> bool {
        self.walls.get(x, y).unwrap_or(true)
    }

    pub fn lose(&mut self) {
        self.lose = true;
    }

    pub fn game_over(&self) -> bool {
        if self.win {
            println!("Pacman won! Score: {}", self.score);
            true
        } else if self.lose {
            println!("Pacman lost! Score: {}", self.score);
            true
        } else {
            false
        }
    }
}
