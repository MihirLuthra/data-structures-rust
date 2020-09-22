use super::linked_list::*;

fn get_new_linked_list_with_values<T: Copy>(vec: &Vec<T>) -> LinkedList<T> {
    let mut linked_list = LinkedList::<T>::new();

    for element in vec.iter() {
        linked_list.append(*element);
    }

    return linked_list;
}

#[test]
fn iter_mut_test() {
    let test_vector = vec![1, 2, 3, 4, 5];

    let mut linked_list = get_new_linked_list_with_values(&test_vector);

    let mut iter_mut = linked_list.iter_mut();

    for element in test_vector.iter() {
        let next = iter_mut.next().unwrap();
        *next += 1;
        assert_eq!(*element + 1, *next);
    }
}

#[test]
fn iter_test() {
    let test_vector = vec![1, 2, 3, 4, 5];

    let linked_list = get_new_linked_list_with_values(&test_vector);

    let mut iter = linked_list.iter();

    for element in test_vector.iter() {
        let next = iter.next().unwrap();
        assert_eq!(*element, *next);
    }
}

#[test]
fn into_iter_test() {
    let test_vector = vec![1, 2, 3, 4, 5];

    let linked_list = get_new_linked_list_with_values(&test_vector);

    let mut into_iter = linked_list.into_iter();

    for element in test_vector.iter() {
        let next = into_iter.next().unwrap();
        assert_eq!(*element, next);
    }
}

#[test]
fn fmt_display_test() {
    let test_vector = vec![1, 2, 3, 4, 5];

    let linked_list = get_new_linked_list_with_values(&test_vector);

    assert_eq!(
        format!("{}", linked_list),
        "HEAD -> 1 -> 2 -> 3 -> 4 -> 5 -> None"
    );
}

#[test]
fn delete_at_posn_test() {
    let test_vector = vec![1, 2, 3, 4, 5];

    let mut linked_list = get_new_linked_list_with_values(&test_vector);

    linked_list
        .delete_at_posn(2)
        .expect("Error while deleting from 2nd posn");
    assert_eq!(
        format!("{}", linked_list),
        "HEAD -> 1 -> 2 -> 4 -> 5 -> None"
    );

    linked_list
        .delete_at_posn(2)
        .expect("Error while deleting from 2nd posn");
    assert_eq!(format!("{}", linked_list), "HEAD -> 1 -> 2 -> 5 -> None");

    if let Ok(_) = linked_list.delete_at_posn(3) {
        panic!("Should have caused error as we are trying to delete at a posn greater than length");
    }

    assert_eq!(linked_list.length, 3);
}

#[test]
fn delete_where_test() {
    let test_vector = vec![1, 2, 3, 4, 5];

    let mut linked_list = get_new_linked_list_with_values(&test_vector);

    let mut counter = 3;
    linked_list
        .delete_where(move |_element| {
            counter -= 1;
            counter > -1
        })
        .expect("Unexpected error");

    assert_eq!(format!("{}", linked_list), "HEAD -> 4 -> 5 -> None");
}
