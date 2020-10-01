use super::bst::*;
use super::fmt_display::*;

#[test]
fn insert() {
    let mut bst = Bst::<i32>::new();

    bst.insert(55);
    bst.insert(5);
    bst.insert(27);
    bst.insert(100);
    bst.insert(23);

    assert_eq!(format!("{}", bst), "5 23 27 55 100 ");
}

#[test]
fn delete() {
    let mut bst = Bst::<i32>::new();

    bst.insert(15);
    bst.insert(22);
    bst.insert(7);
    bst.insert(5);
    bst.insert(11);
    bst.insert(9);
    bst.insert(13);
    bst.insert(8);
    bst.insert(10);
    bst.insert(12);
    bst.insert(14);

    assert_eq!(format!("{}", bst), "5 7 8 9 10 11 12 13 14 15 22 ");

    bst.delete(11);
    assert_eq!(format!("{}", bst), "5 7 8 9 10 12 13 14 15 22 ");

    bst.delete(22);
    assert_eq!(format!("{}", bst), "5 7 8 9 10 12 13 14 15 ");

    bst.delete(10);
    assert_eq!(format!("{}", bst), "5 7 8 9 12 13 14 15 ");

    bst.delete(9);
    assert_eq!(format!("{}", bst), "5 7 8 12 13 14 15 ");

    bst.delete(15);
    assert_eq!(format!("{}", bst), "5 7 8 12 13 14 ");
}

#[test]
fn search() {
    let mut bst = Bst::<i32>::new();

    assert_eq!(bst.search(1), false);

    bst.insert(15);
    bst.insert(22);
    bst.insert(7);
    bst.insert(5);
    bst.insert(11);
    bst.insert(9);
    bst.insert(13);
    bst.insert(8);
    bst.insert(10);
    bst.insert(12);
    bst.insert(14);

    assert_eq!(format!("{}", bst), "5 7 8 9 10 11 12 13 14 15 22 ");

    assert_eq!(bst.search(1), false);
    assert_eq!(bst.search(5), true);
    assert_eq!(bst.search(7), true);
    assert_eq!(bst.search(8), true);
    assert_eq!(bst.search(9), true);
    assert_eq!(bst.search(10), true);
    assert_eq!(bst.search(11), true);
    assert_eq!(bst.search(12), true);
    assert_eq!(bst.search(13), true);
    assert_eq!(bst.search(14), true);
    assert_eq!(bst.search(15), true);
    assert_eq!(bst.search(22), true);

    bst.delete(11);
    assert_eq!(format!("{}", bst), "5 7 8 9 10 12 13 14 15 22 ");
    assert_eq!(bst.search(11), false);

    bst.delete(22);
    assert_eq!(format!("{}", bst), "5 7 8 9 10 12 13 14 15 ");
    assert_eq!(bst.search(22), false);

    bst.delete(10);
    assert_eq!(format!("{}", bst), "5 7 8 9 12 13 14 15 ");
    assert_eq!(bst.search(10), false);

    bst.delete(9);
    assert_eq!(format!("{}", bst), "5 7 8 12 13 14 15 ");
    assert_eq!(bst.search(9), false);

    bst.delete(15);
    assert_eq!(format!("{}", bst), "5 7 8 12 13 14 ");
    assert_eq!(bst.search(15), false);
}
