pub mod ghost;
pub mod pacman;

use super::game::Configuration;
use super::game::Direction;
use super::game::Grid;

pub fn possible_actions(config: &Configuration, walls: &Grid) -> Vec<Direction> {
    let position = config.position();
    let x = position.0 as isize;
    let y = position.1 as isize;

    Direction::values()
        .into_iter()
        .filter(|direction| {
            let (dx, dy) = direction.to_diff(1);
            let new_x = (x + dx) as usize;
            let new_y = (y + dy) as usize;
            !walls.get(new_x, new_y).unwrap_or(true)
        })
        .collect()
}
