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
