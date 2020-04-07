mod change_orientation;
mod player_idle;
mod player_move;
mod player_punch;
mod player_walk;

pub use self::{
	change_orientation::ChangeOrientation, player_idle::PlayerIdle, player_move::PlayerMove,
	player_punch::PlayerPunch, player_walk::PlayerWalk,
};
