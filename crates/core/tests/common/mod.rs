use rand::prelude::*;
use std::collections::HashSet;

pub fn is_subset(sub: &[u32], set: &HashSet<u32>) -> bool {
    for elem in sub {
        if !set.contains(elem) {
            return false;
        }
    }
    true
}

pub fn is_minimal_hitting_set(
    candidate_hs: &[u32],
    target_sets: &[Vec<u32>],
    blocked_sets: &[Vec<u32>],
) -> bool {
    let candidate_set: HashSet<u32> = candidate_hs.iter().copied().collect();

    for blocked in blocked_sets {
        if is_subset(blocked, &candidate_set) {
            return false;
        }
    }

    let mut required_elements = HashSet::new();

    for target in target_sets {
        let mut only: Option<u32> = None;
        let mut hit_twice = false;

        for elem in target {
            if !candidate_set.contains(elem) {
                continue;
            }
            match only {
                None => only = Some(*elem),
                Some(prev) if prev == *elem => {}
                Some(_) => {
                    hit_twice = true;
                    break;
                }
            }
        }
        match (only, hit_twice) {
            (None, _) => return false,
            (Some(e), false) => {
                required_elements.insert(e);
            }
            (Some(_), true) => {}
        }
    }

    required_elements.len() == candidate_set.len()
}

pub fn is_same_collection_of_set(a: &[Vec<u32>], b: &[Vec<u32>]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut a_sorted = a.to_vec();
    let mut b_sorted = b.to_vec();

    for vec in &mut a_sorted {
        vec.sort_unstable();
    }

    for vec in &mut b_sorted {
        vec.sort_unstable();
    }

    a_sorted.sort();
    b_sorted.sort();

    a_sorted == b_sorted
}

pub fn random_instance(
    rng: &mut StdRng,
    size_collection: usize,
    max_set_size: usize,
    max_element: u32,
) -> Vec<Vec<u32>> {
    let mut collection_sets: Vec<Vec<u32>> = Vec::with_capacity(size_collection);
    for _ in 0..size_collection {
        let many_elements = rng.random_range(1..max_set_size + 1);
        let mut set: Vec<u32> = Vec::with_capacity(many_elements);
        for _ in 0..many_elements {
            set.push(rng.random_range(0..max_element));
        }
        collection_sets.push(set);
    }
    collection_sets
}

pub fn brute_force_minimal_hitting_sets(sets: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut elems: Vec<u32> = sets.iter().flatten().copied().collect();
    elems.sort_unstable();
    elems.dedup();
    assert!(
        elems.len() <= 20,
        "too many elements for brute force: {}",
        elems.len()
    );

    let mut result = Vec::new();
    for mask in 0u32..(1 << elems.len()) {
        let subset: Vec<u32> = (0..elems.len())
            .filter(|&i| mask & (1 << i) != 0)
            .map(|i| elems[i])
            .collect();
        if is_minimal_hitting_set(&subset, sets, &[]) {
            result.push(subset);
        }
    }
    result
}
