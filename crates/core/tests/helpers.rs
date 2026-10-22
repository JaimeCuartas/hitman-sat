mod common;
use common::{is_minimal_hitting_set, is_same_collection_of_set, is_subset};
use std::collections::HashSet;

#[test]
fn is_minimal_hitting_set_correct() {
    let collection_sets = vec![vec![1, 2], vec![2, 3], vec![3]];
    assert!(!is_minimal_hitting_set(&[1, 2, 3], &collection_sets, &[]));
    assert!(!is_minimal_hitting_set(&[1, 2], &collection_sets, &[]));
    assert!(!is_minimal_hitting_set(&[3], &collection_sets, &[]));
    // It is not because it should be blocked
    assert!(!is_minimal_hitting_set(
        &[1, 3],
        &collection_sets,
        &[vec![3, 1]]
    ));
    assert!(is_minimal_hitting_set(&[1, 3], &collection_sets, &[]));
    assert!(is_minimal_hitting_set(
        &[2, 3],
        &collection_sets,
        &[vec![3, 1]]
    ));
}

#[test]
fn is_minimal_hitting_set_correct_duplicates() {
    let collection_sets = vec![vec![1, 1, 2], vec![2, 3], vec![3]];
    assert!(is_minimal_hitting_set(&[1, 3], &collection_sets, &[]));
    let collection_sets = vec![vec![1, 2, 1], vec![2, 3], vec![3]];
    assert!(is_minimal_hitting_set(&[1, 3], &collection_sets, &[]));
}

#[test]
fn is_subset_correct() {
    assert!(is_subset(&[1, 2, 3], &HashSet::from([3, 2, 1])));
    assert!(is_subset(&[], &HashSet::from([3, 2, 1])));
    assert!(!is_subset(&[4], &HashSet::from([3, 2, 1])));
    assert!(is_subset(&[1], &HashSet::from([3, 2, 1])));
    assert!(is_subset(&[3, 1], &HashSet::from([3, 2, 1])));
    assert!(!is_subset(&[3, 1, 4], &HashSet::from([3, 2, 1])));
}

#[test]
fn is_same_collection_of_set_correct() {
    assert!(is_same_collection_of_set(
        &[],
        &[]
    ));
    assert!(is_same_collection_of_set(
        &[vec![1, 3], vec![2, 3]],
        &[vec![1, 3], vec![2, 3]]
    ));
    assert!(is_same_collection_of_set(
        &[vec![1, 3], vec![2, 3]],
        &[vec![2, 3], vec![1, 3]]
    ));
    assert!(!is_same_collection_of_set(
        &[vec![1, 3], vec![2, 3]],
        &[vec![2, 3]]
    ));
    assert!(!is_same_collection_of_set(
        &[vec![1, 3], vec![2, 3]],
        &[vec![2, 3], vec![1]]
    ));
}
