mod common;
use common::{
    brute_force_minimal_hitting_sets, is_minimal_hitting_set, is_same_collection_of_set, is_subset,
    random_instance,
};
use rand::prelude::*;
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
    assert!(is_same_collection_of_set(&[], &[]));
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

#[test]
fn brute_force_finds_all_minimal_hitting_sets() {
    let sets = vec![vec![1, 2], vec![2, 3], vec![3]];
    let found = brute_force_minimal_hitting_sets(&sets);
    assert!(
        is_same_collection_of_set(&found, &[vec![1, 3], vec![2, 3]]),
        "got {found:?}"
    );
}

#[test]
fn brute_force_edge_cases() {
    assert_eq!(brute_force_minimal_hitting_sets(&[]), vec![Vec::new()]);

    let empty_set: Vec<Vec<u32>> = Vec::new();
    assert_eq!(
        brute_force_minimal_hitting_sets(&[vec![1, 2], vec![]]),
        empty_set
    );

    let found = brute_force_minimal_hitting_sets(&[vec![1, 1, 2]]);
    assert!(
        is_same_collection_of_set(&found, &[vec![1], vec![2]]),
        "got {found:?}"
    );
}

#[test]
fn random_instance_respects_bounds_and_seed() {
    let mut rng = StdRng::seed_from_u64(1);
    let sets = random_instance(&mut rng, 50, 4, 10);
    assert_eq!(sets.len(), 50);
    for set in &sets {
        assert!((1..=4).contains(&set.len()), "bad size: {set:?}");
        assert!(set.iter().all(|&e| e < 10), "element out of range: {set:?}");
    }

    let mut rng_again = StdRng::seed_from_u64(1);
    assert_eq!(sets, random_instance(&mut rng_again, 50, 4, 10));
}
