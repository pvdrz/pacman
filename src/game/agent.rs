use super::Direction;

pub trait Agent {
    fn get_action(&self, state: GameState) -> Direction;
    fn get_index(&self) -> usize;
}
