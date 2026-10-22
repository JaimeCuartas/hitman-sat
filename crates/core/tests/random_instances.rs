mod common;
use common::{is_minimal_hitting_set, is_same_collection_of_set};
use hitman_sat::Hitman;

#[test]
fn get_hitting_set() {
    let target_sets: Vec<Vec<u32>> = vec![vec![1, 2], vec![2, 3], vec![3]];
    let mut hitman = Hitman::new();
    for s in &target_sets {
        hitman.hit(s).unwrap();
    }

    let mut blocked_sets: Vec<Vec<u32>> = Vec::new();
    while let Some(hs) = hitman.get().unwrap() {
        assert!(
            is_minimal_hitting_set(&hs, &target_sets, &blocked_sets),
            "{hs:?} is not minimal hitting set"
        );
        hitman.block(&hs).expect("Should not return an Error");
        blocked_sets.push(hs);
    }

    let expected_hitting_sets: Vec<Vec<u32>> = vec![vec![3, 1], vec![3, 2]];

    assert!(
        is_same_collection_of_set(&expected_hitting_sets, &blocked_sets),
        "The expected collection of hitting sets was not met"
    );
}
