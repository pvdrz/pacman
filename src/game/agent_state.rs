use game::Configuration;

#[derive(Clone)]
pub struct AgentState {
    start: Configuration,
    configuration: Configuration,
    is_pacman: bool,
    scared_timer: usize,
}

impl AgentState {
    pub fn new(start: Configuration, is_pacman: bool) -> Self {
        AgentState {
            start: start.clone(),
            configuration: start,
            is_pacman,
            scared_timer: 0,
        }
    }

    pub fn config(&self) -> &Configuration {
        &self.configuration
    }

    pub fn timer(&self) -> usize {
        self.scared_timer
    }

    pub fn update_config(&mut self, config: Configuration) {
        self.configuration = config;
    }

    pub fn scare(&mut self, time: usize) {
        self.scared_timer = time;
    }

    pub fn decrement_timer(&mut self) {
        self.scared_timer = self.scared_timer.saturating_sub(1);
    }

    pub fn reset(&mut self) {
        self.scared_timer = 0;
        self.configuration = self.start.clone();
    }
}
