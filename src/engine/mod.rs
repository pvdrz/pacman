mod quiet;
mod text;

pub use self::quiet::Quiet;
pub use self::text::Text;

use game::GameState;

pub trait Engine {
    fn update(&mut self, state: &GameState);
}
