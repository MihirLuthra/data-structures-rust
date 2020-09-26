use super::bst::*;

#[test]
fn insert() {
    let mut bst = Bst::<i32>::new();

    bst.insert(55);
    bst.insert(5);
    bst.insert(27);
    bst.insert(100);
    bst.insert(23);

    assert_eq!(format!("{}", bst), "5 23 27 55 100");
}
