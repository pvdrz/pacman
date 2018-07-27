use game::Direction;
use game::GameState;

pub fn legal_actions(state: &GameState, index: usize) -> Vec<Direction> {
    let config = state.agent_state(index).unwrap().config();
    let reverse = config.direction().reverse();
    let walls = state.walls();

    let mut actions = super::possible_actions(config, walls);

    actions.remove_item(&Direction::Stop);
    if actions.len() > 1 {
        actions.remove_item(&reverse);
    }
    actions
}

pub fn apply_action(state: &mut GameState, action: Direction, index: usize) {
    if legal_actions(state, index).contains(&action) {
        let (dx, dy) = action.to_diff(1);
        let ghost_state = state.agent_state_mut(index).unwrap();
        let config = ghost_state.config().gen_successor(dx, dy);
        ghost_state.update_config(config);
        ghost_state.decrement_timer();
    }
}

pub fn check_death(state: &mut GameState, index: usize) -> isize {
    let mut score_change: isize = 0;
    let pacman_position = state.agent_state(0).unwrap().config().position();

    if index == 0 {
        for ghost_index in 1..state.num_agents() {
            let ghost_position = state.agent_position(ghost_index).unwrap();
            if pacman_position == ghost_position {
                score_change += collide(state, ghost_index);
            }
        }
    } else {
        let ghost_position = state.agent_position(index).unwrap();
        if pacman_position == ghost_position {
            score_change += collide(state, index);
        }
    }
    score_change
}

fn collide(state: &mut GameState, index: usize) -> isize {
    let ghost_timer = state.agent_state(index).unwrap().timer();

    if ghost_timer > 0 {
        state.agent_state_mut(index).unwrap().reset();
        200
    } else {
        state.lose();
        -500
    }
}
