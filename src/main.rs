mod tictactoe;

use tictactoe::Board;

fn main() {
	let mut board: Board = Default::default();
	
	board.draw();
	board.process_turn('x', "a0");
	board.process_turn('x', "b1");
	board.process_turn('x', "c2");
	board.draw();
}
