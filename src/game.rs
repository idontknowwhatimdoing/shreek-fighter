use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, NullStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
	formats::texture::ImageFormat, Camera, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
};
use amethyst::window::ScreenDimensions;

pub struct Game;

impl SimpleState for Game {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;
		world.register::<Moveable>();

		let dim = (*world.read_resource::<ScreenDimensions>()).clone();
		init_camera(world, dim);

		let shrek_spritesheet = load_shrek_spritesheet(world);
		init_shrek(world, shrek_spritesheet);
	}
}

fn init_camera(world: &mut World, dim: ScreenDimensions) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(dim.width() / 2.0, dim.height() / 2.0, 1.0);

	world
		.create_entity()
		.with(transform)
		.with(Camera::standard_2d(dim.width(), dim.height()))
		.build();
}

fn load_shrek_spritesheet(world: &World) -> Handle<SpriteSheet> {
	let loader = world.read_resource::<Loader>();
	let texture_handle = loader.load(
		"assets/textures/shrek_spritesheet.png",
		ImageFormat::default(),
		(),
		&world.read_resource::<AssetStorage<Texture>>(),
	);

	loader.load(
		"assets/textures/shrek_spritesheet.ron",
		SpriteSheetFormat(texture_handle),
		(),
		&world.read_resource::<AssetStorage<SpriteSheet>>(),
	)
}

fn init_shrek(world: &mut World, spritesheet_handle: Handle<SpriteSheet>) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(50.0, 200.0, 0.0);

	let sprite_render = SpriteRender {
		sprite_sheet: spritesheet_handle,
		sprite_number: 0,
	};

	world
		.create_entity()
		.with(transform)
		.with(Moveable)
		.with(sprite_render)
		.build();
}

// ---[[ COMPONENTS ]]---

#[derive(Default)]
struct Moveable;

impl Component for Moveable {
	type Storage = NullStorage<Self>;
}
