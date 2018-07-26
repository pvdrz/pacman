use game::Direction;
use game::GameState;

use failure::err_msg;
use failure::Error;

pub fn legal_actions(state: &GameState) -> Result<Vec<Direction>, Error> {
    let config = state
        .agent_state(0)
        .ok_or(err_msg("Invalid index"))?
        .config();
    let walls = state.walls();
    Ok(super::possible_actions(config, walls))
}

pub fn apply_action(state: &mut GameState, action: Direction) -> Result<(), Error> {
    if legal_actions(state)?.contains(&action) {
        let (dx, dy) = action.to_diff(1);
        let pacman_state = state.agent_state_mut(0).ok_or(err_msg("Invalid index"))?;
        let config = pacman_state.config().gen_successor(dx, dy);
        pacman_state.update_config(config);
    }
    Ok(())
}
