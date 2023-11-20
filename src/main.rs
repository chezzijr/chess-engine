use chess::Board;

fn main() {
    let mut board = Board::default();
    let moves = ["c4", "d6", "Qa4", "c6", "Qxc6", "qd7", "Qxd7", "kxd7"];
    for m in moves.iter() {
        println!("{}", board);
        board.make_move((*m).to_owned()).unwrap();
    }
    println!("{}", board);
}
