mod common;
use common::{
    brute_force_minimal_hitting_sets, is_minimal_hitting_set, is_same_collection_of_set,
    random_instance,
};
use hitman_sat::Hitman;
use rand::prelude::*;

// Bootstrap with...
fn hitman_for(sets: &[Vec<u32>]) -> Hitman {
    let mut hitman = Hitman::new();
    for set in sets {
        hitman.hit(set).unwrap();
    }
    hitman
}

fn enumerate_checked(sets: &[Vec<u32>], limit: usize) -> Vec<Vec<u32>> {
    let mut hitman = hitman_for(sets);
    let mut found: Vec<Vec<u32>> = Vec::new();
    while let Some(hs) = hitman.get().unwrap() {
        assert!(
            is_minimal_hitting_set(&hs, sets, &found),
            "{hs:?} is not minimal hitting set"
        );
        hitman.block(&hs).unwrap();
        found.push(hs);
        if found.len() == limit {
            break;
        }
    }
    found
}

#[test]
fn get_hitting_set_known() {
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

#[test]
fn get_hitting_sets_sparse_random() {
    for seed in 0..100 {
        let mut rng = StdRng::seed_from_u64(1_000 + seed);
        let sets: Vec<Vec<u32>> = random_instance(&mut rng, 6, 8, 3)
            .into_iter()
            .map(|set| set.into_iter().map(|e| e * 104_729 + 17).collect())
            .collect();

        let found = enumerate_checked(&sets, usize::MAX);
        let expected = brute_force_minimal_hitting_sets(&sets);
        assert!(is_same_collection_of_set(&expected, &found));
    }
}

#[test]
fn once_finished_gets_none() {
    let mut rng = StdRng::seed_from_u64(99);
    let sets = random_instance(&mut rng, 5, 6, 3);
    let mut hitman = hitman_for(&sets);
    while let Some(hs) = hitman.get().unwrap() {
        hitman.block(&hs).unwrap();
    }
    for _ in 0..3 {
        assert_eq!(hitman.get().unwrap(), None);
    }
}

#[test]
fn get_hitting_sets_small_random() {
    for seed in 0..300 {
        let mut rng = StdRng::seed_from_u64(seed);
        let num_sets = rng.random_range(1..=8);
        let num_elems = rng.random_range(1..=10);
        let max_set_size = rng.random_range(1..=4);
        let sets = random_instance(&mut rng, num_sets, num_elems, max_set_size);

        let found = enumerate_checked(&sets, usize::MAX);
        let expected = brute_force_minimal_hitting_sets(&sets);
        assert!(
            is_same_collection_of_set(&expected, &found),
        );
    }
}

#[test]
fn get_hitting_sets_mid_random() {
    for seed in 0..100 {
        let mut rng = StdRng::seed_from_u64(10_000 + seed);
        let sets = random_instance(&mut rng, 60, 40, 6);
        let found = enumerate_checked(&sets, 200);
        assert!(
            !found.is_empty()
        );
    }
}

#[test]
fn get_hitting_sets_large_random() {
    for seed in 0..20 {
        let mut rng = StdRng::seed_from_u64(20_000 + seed);
        let sets = random_instance(&mut rng, 400, 200, 10);
        let found = enumerate_checked(&sets, 2_000);
        assert!(
            !found.is_empty(),
            "seed {}: no hitting set found",
            20_000 + seed
        );
    }
}
