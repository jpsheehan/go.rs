use crate::go_core::*;

#[test]
fn test_get_adjacent() {
    let mut b = GoBoard::new(19);

    b.place(Point::new(0, 0));
    assert_eq!(b.get_adjacent(Point::new(0, 0)).len(), 2);

    b.place(Point::new(0, 5));
    assert_eq!(b.get_adjacent(Point::new(0, 5)).len(), 3);

    b.place(Point::new(10, 10));
    assert_eq!(b.get_adjacent(Point::new(10, 10)).len(), 4);
}

#[test]
fn test_get_group() {
    let mut b = GoBoard::new(19);
    let mut group = Vec::new();

    b.place(Point::new(5, 5));
    b.get_group(CellState::Black, Point::new(5, 5), &mut group);
    assert_eq!(group.len(), 1);

    b.turn = CellState::Black;
    group.clear();
    b.place(Point::new(5, 6));
    b.get_group(CellState::Black, Point::new(5, 5), &mut group);
    assert_eq!(group.len(), 2);
}

#[test]
fn test_suicide() {
    let mut b = GoBoard::new(19);
    b.place(Point::new(1, 0));
    b.place(Point::new(5, 5));
    b.place(Point::new(0, 1));
    b.place(Point::new(0, 0));

    assert_eq!(b.get(Point::new(0, 0)), CellState::None)
}

#[test]
fn test_capture() {
    let mut b = GoBoard::from_str(
        "
    .....
    .BBB.
    BWWW.
    .BBB.
    .....
    ",
        CellState::Black,
    );
    b.place(Point::new(4, 2));

    assert_eq!(b.get(Point::new(1, 2)), CellState::None);
    assert_eq!(b.get(Point::new(2, 2)), CellState::None);
    assert_eq!(b.get(Point::new(3, 2)), CellState::None);
}
