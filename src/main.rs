mod go_core;
use go_core::*;

fn main() {
    let mut board = GoBoard::new(9);
    board.place(Point::new(2, 2));
    board.place(Point::new(2, 3));
    board.place(Point::new(2, 3));
    board.place(Point::new(3, 3));
    board.place(Point::new(4, 4));
    board.place(Point::new(1, 3));
    board.place(Point::new(5, 5));
    board.place(Point::new(2, 4));
    board.print();
}
