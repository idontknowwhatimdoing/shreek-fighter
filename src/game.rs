// mod components;

use amethyst::prelude::*;

pub struct Game;

impl SimpleState for Game {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let mut world = data.world;

		// initialize_background();
		// initialize_players();
	}
}
