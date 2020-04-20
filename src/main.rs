mod game;
mod systems;

use amethyst::{
	core::transform::TransformBundle,
	input::{InputBundle, StringBindings},
	prelude::*,
	renderer::{
		plugins::{RenderFlat2D, RenderToWindow},
		types::DefaultBackend,
		RenderingBundle,
	},
	utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());

	let app_root = application_root_dir()?;

	let assets_dir = app_root.join("assets");
	let config_dir = app_root.join("config");
	let display_config_path = config_dir.join("display.ron");

	let binding_path = app_root.join("config").join("bindings.ron");

	let input_bundle =
		InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

	let game_data = GameDataBuilder::default()
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				.with_plugin(RenderToWindow::from_config_path(display_config_path)?)
				.with_plugin(RenderFlat2D::default()),
		)?
		.with_bundle(TransformBundle::new())?
		.with_bundle(input_bundle)?
		.with(systems::ShrekMove, "shrek_move", &["input_system"])
		.with(
			systems::ShrekIdle::default(),
			"shrek_idle",
			&["input_system"],
		)
		.with(
			systems::ShrekWalk::default(),
			"shrek_walk",
			&["input_system"],
		)
		.with(
			systems::ChangeShrekOrientation,
			"change_shrek_orientation",
			&["input_system"],
		)
		.with(
			systems::ShrekPunch::default(),
			"shrek_punch",
			&["input_system"],
		)
		.with(
			systems::ShrekJump::default(),
			"shrek_jump",
			&["input_system"],
		)
		.with(systems::GuardMove, "guard_move", &["input_system"])
		.with(
			systems::ChangeGuardOrientation,
			"change_guard_orientation",
			&["input_system"],
		)
		.with(
			systems::GuardWalk::default(),
			"guard_walk",
			&["input_system"],
		)
		.with(
			systems::GuardJump::default(),
			"guard_jump",
			&["input_system"],
		)
		.with(
			systems::GuardPunch::default(),
			"guard_punch",
			&["input_system"],
		)
		.with(
			systems::GuardIdle::default(),
			"guard_idle",
			&["input_system"],
		);

	let mut game = Application::new(assets_dir, game::Game, game_data)?;
	game.run();

	Ok(())
}
