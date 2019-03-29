mod tictactoe;

use tictactoe::Game;
use tictactoe::State;

fn main() {
	let mut game: Game = Default::default();
	while game.state == State::InProgress {
		game.take_turn();
	}
	println!("\nggwp: {}", game.state);
}
