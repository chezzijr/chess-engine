use chess::Board;

fn main() {
    let mut board = Board::default();
    loop {
        println!("{}\n{:?} to move", board, board.turn);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        board.make_move(input.trim().to_owned()).unwrap();
    }
}
