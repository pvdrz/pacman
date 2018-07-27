use game::Direction;
use game::GameState;

pub fn legal_actions(state: &GameState) -> Vec<Direction> {
    let config = state.agent_state(0).unwrap().config();
    let walls = state.walls();
    super::possible_actions(config, walls)
}

pub fn apply_action(state: &mut GameState, action: Direction) {
    if legal_actions(state).contains(&action) {
        let (dx, dy) = action.to_diff(1);
        let pacman_state = state.agent_state_mut(0).unwrap();
        let config = pacman_state.config().gen_successor(dx, dy);
        pacman_state.update_config(config);
    }
}
