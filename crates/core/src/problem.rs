use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;

use crate::HitmanError;

// threshold for hitting sets for the
// functinon hit used when we need to
// filter duplication, different strategies
// with this constant
// TODO: test DEDUP_THRESHOLD
const DEDUP_THRESHOLD: usize = 100;

pub struct Hitman {
    sets: Vec<Vec<u32>>,
    var2id: HashMap<u32, u32>,
    id2var: Vec<u32>,
}

impl Default for Hitman {
    fn default() -> Self {
        Self::new()
    }
}

impl Hitman {
    #[must_use]
    pub fn new() -> Self {
        Self {
            sets: Vec::new(),
            var2id: HashMap::new(),
            id2var: Vec::new(),
        }
    }

    // TODO when hitting an empty set the collection
    // has no more hitting sets. I need to cover this
    /// Add the set `claues` as a new set to hit
    /// # Errors
    /// Returns [`HitmanError`] if the number of
    /// lits is greater than u32 type
    pub fn hit(&mut self, clause: &[u32]) -> Result<(), HitmanError> {
        let mut clause_ids: Vec<u32> = Vec::with_capacity(clause.len());

        for element in clause {
            let id = self.get_id(*element)?;
            clause_ids.push(id);
        }
        self.sets.push(clause_ids);
        Ok(())
    }

    /// Add the set `claues` as a new set to hit
    /// if it has duplicates, it filters them
    /// # Errors
    /// Returns [`HitmanError`] if the number of
    /// lits is greater than u32 type
    pub fn hit_filtering(&mut self, clause: &[u32]) -> Result<(), HitmanError> {
        let clause_ids: Vec<u32> = if clause.len() > DEDUP_THRESHOLD {
            let mut seen = HashSet::with_capacity(clause.len());
            let mut ids = Vec::with_capacity(clause.len());

            for &element in clause {
                if seen.insert(element) {
                    let id = self.get_id(element)?;
                    ids.push(id);
                }
            }
            ids
        } else {
            let mut ids: Vec<u32> = Vec::with_capacity(clause.len());

            for (i, element) in clause.iter().enumerate() {
                if !clause[..i].contains(element) {
                    let id = self.get_id(*element)?;
                    ids.push(id);
                }
            }
            ids
        };
        self.sets.push(clause_ids);
        Ok(())
    }

    fn get_id(&mut self, var_name: u32) -> Result<u32, HitmanError> {
        let id_entry = self.var2id.entry(var_name);
        match id_entry {
            Entry::Occupied(occupied) => Ok(*occupied.get()),
            Entry::Vacant(vacant) => {
                let size_ids = self.id2var.len() + 1;
                let new_id =
                    u32::try_from(size_ids).map_err(|_| HitmanError::OutOfRange {
                        value: self.id2var.len() + 1,
                    })?;
                self.id2var.push(var_name);
                vacant.insert(new_id);
                Ok(new_id)
            }
        }
    }

    #[must_use]
    pub fn sets(&self) -> &[Vec<u32>] {
        &self.sets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let hitman = Hitman::new();
        assert_eq!(hitman.id2var.len(), 0);
    }
    #[test]
    fn hit_assigns_ids() {
        let mut hitman = Hitman::new();
        hitman.hit(&[28, 10]).unwrap();
        hitman.hit(&[28, 11]).unwrap();
        hitman.hit(&[29, 28, 30, 10]).unwrap();
        assert_eq!(hitman.var2id.get(&28u32), Some(&1u32));
        assert_eq!(hitman.var2id.get(&10u32), Some(&2u32));
        assert_eq!(hitman.var2id.get(&11u32), Some(&3u32));
        assert_eq!(hitman.var2id.get(&29u32), Some(&4u32));
        assert_eq!(hitman.var2id.get(&30u32), Some(&5u32));
        assert_eq!(hitman.id2var.len(), 5);
    }
    #[test]
    fn hit_assigns_creates_sets() {
        let mut hitman = Hitman::new();
        hitman.hit(&[28, 10]).unwrap();
        hitman.hit(&[28, 11]).unwrap();
        hitman.hit(&[29, 28, 30, 10]).unwrap();
        assert_eq!(hitman.sets, vec![vec![1, 2], vec![1, 3], vec![4, 1, 5, 2]]);
    }
    #[test]
    fn empty_clause() {
        let mut hitman = Hitman::new();
        hitman.hit(&[]).unwrap();
        assert_eq!(hitman.id2var.len(), 0);
        assert_eq!(hitman.sets.len(), 1);
    }
    #[test]
    fn id_round_trip() {
        let mut hitman = Hitman::new();
        hitman.hit(&[28, 10]).unwrap();
        hitman.hit(&[28, 11]).unwrap();
        hitman.hit(&[29, 28, 30, 10]).unwrap();
        assert_eq!(hitman.id2var, &[28, 10, 11, 29, 30]);
    }
    #[test]
    fn test_get_sets() {
        let mut hitman = Hitman::new();
        hitman.hit(&[5, 3, 10]).unwrap();
        let first = hitman.sets()[0].clone();
        hitman.hit(&[2, 3, 4]).unwrap();
        assert_eq!(first, &[1, 2, 3]);
    }
    #[test]
    fn hit_and_hit_w_filter() {
        let mut hitman = Hitman::new();
        hitman.hit(&[2, 2, 3]).unwrap();
        assert_eq!(hitman.sets, vec![vec![1, 1, 2]]);
        let mut hitman = Hitman::new();
        hitman.hit_filtering(&[2, 2, 3]).unwrap();
        assert_eq!(hitman.sets, vec![vec![1, 2]]);
    }
}
