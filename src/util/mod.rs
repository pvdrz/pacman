use game::GameState;

pub fn manhattan_distance(pos_a: (usize, usize), pos_b: (usize, usize)) -> usize {
    let dx = (pos_a.0 as isize - pos_b.0 as isize).abs() as usize;
    let dy = (pos_a.1 as isize - pos_b.1 as isize).abs() as usize;
    dx + dy
}

pub fn maze_distance(
    state: &GameState,
    index: usize,
    position: (usize, usize),
    depth: usize,
) -> usize {
    if state.agent_position(index).unwrap() == position || depth == 0 {
        0
    } else {
        state
            .legal_actions(index)
            .into_iter()
            .filter_map(|a| {
                state
                    .gen_successor(index, a)
                    .map(|succ| maze_distance(&succ, index, position, depth - 1))
                    .ok()
            })
            .min()
            .unwrap_or(usize::max_value())
    }
}
