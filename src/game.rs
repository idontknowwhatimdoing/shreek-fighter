use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, NullStorage, VecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
	formats::texture::ImageFormat, Camera, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
};

pub const ARENA_WIDTH: f32 = 563.33;
pub const ARENA_HEIGHT: f32 = 330.0;
pub const SHREK_WIDTH: f32 = 33.0;
pub const SHREK_HEIGHT: f32 = 46.0;
pub const GUARD_WIDTH: f32 = 26.0;
pub const GUARD_HEIGHT: f32 = 46.0;
pub const GROUND_HEIGHT: f32 = 40.0;
pub const JUMP_HEIGHT: f32 = 250.0;

pub struct Game;

impl SimpleState for Game {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		init_camera(world);

		let background_spritesheet = load_bg_spritesheet(world);
		init_background(world, background_spritesheet);

		let player_spritesheet = load_shrek_spritesheet(world);
		let guard_spritesheet = load_guard_spritesheet(world);
		init_players(world, player_spritesheet, guard_spritesheet);
	}
}

fn load_bg_spritesheet(world: &mut World) -> Handle<SpriteSheet> {
	let loader = world.read_resource::<Loader>();
	let texture_handle = loader.load(
		"textures/forest_background_original.png",
		ImageFormat::default(),
		(),
		&world.read_resource::<AssetStorage<Texture>>(),
	);
	loader.load(
		"textures/background.ron",
		SpriteSheetFormat(texture_handle),
		(),
		&world.read_resource::<AssetStorage<SpriteSheet>>(),
	)
}

fn load_shrek_spritesheet(world: &World) -> Handle<SpriteSheet> {
	let loader = world.read_resource::<Loader>();
	let texture_handle = loader.load(
		"textures/shrek_spritesheet.png",
		ImageFormat::default(),
		(),
		&world.read_resource::<AssetStorage<Texture>>(),
	);
	loader.load(
		"textures/shrek_spritesheet.ron",
		SpriteSheetFormat(texture_handle),
		(),
		&world.read_resource::<AssetStorage<SpriteSheet>>(),
	)
}

fn load_guard_spritesheet(world: &World) -> Handle<SpriteSheet> {
	let loader = world.read_resource::<Loader>();
	let texture_handle = loader.load(
		"textures/guard_spritesheet.png",
		ImageFormat::default(),
		(),
		&world.read_resource::<AssetStorage<Texture>>(),
	);
	loader.load(
		"textures/guard_spritesheet.ron",
		SpriteSheetFormat(texture_handle),
		(),
		&world.read_resource::<AssetStorage<SpriteSheet>>(),
	)
}

fn init_camera(world: &mut World) {
	let mut pos = Transform::default();
	pos.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 1.0);

	world
		.create_entity()
		.with(pos)
		.with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
		.build();
}

fn init_players(
	world: &mut World,
	shrek_spritesheet: Handle<SpriteSheet>,
	guard_spritesheet: Handle<SpriteSheet>,
) {
	let mut shrek_pos = Transform::default();
	shrek_pos.set_translation_xyz(0.0, GROUND_HEIGHT, 0.0);
	let mut guard_pos = Transform::default();
	guard_pos.set_translation_xyz(
		ARENA_WIDTH - GUARD_WIDTH / 2.0,
		GROUND_HEIGHT + (SHREK_HEIGHT - GUARD_HEIGHT),
		0.0,
	);

	let shrek_sprite = SpriteRender {
		sprite_sheet: shrek_spritesheet,
		sprite_number: 0,
	};
	let guard_sprite = SpriteRender {
		sprite_sheet: guard_spritesheet,
		sprite_number: 6,
	};

	world
		.create_entity()
		.with(shrek_pos)
		.with(Player::new(Orientation::Right))
		.with(Shrek)
		.with(shrek_sprite)
		.build();
	world
		.create_entity()
		.with(guard_pos)
		.with(Player::new(Orientation::Left))
		.with(Guard)
		.with(guard_sprite)
		.build();
}

fn init_background(world: &mut World, spritesheet_handle: Handle<SpriteSheet>) {
	let mut pos = Transform::default();
	pos.set_translation_xyz(130.0, ARENA_HEIGHT, -1.0);

	let sprite_render = SpriteRender {
		sprite_sheet: spritesheet_handle,
		sprite_number: 0,
	};

	world.create_entity().with(pos).with(sprite_render).build();
}

// ---[[ COMPONENTS ]]---

#[derive(PartialEq)]
pub enum Orientation {
	Left,
	Right,
}

pub struct Player {
	pub orientation: Orientation,
}

impl Player {
	fn new(orientation: Orientation) -> Self {
		Player { orientation }
	}
}

impl Component for Player {
	type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Shrek;

impl Component for Shrek {
	type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Guard;

impl Component for Guard {
	type Storage = NullStorage<Self>;
}
