use super::Configuration;

pub struct AgentState {
    start: Configuration,
    configuration: Configuration,
    is_pacman: bool,
    scared_timer: usize,
}
