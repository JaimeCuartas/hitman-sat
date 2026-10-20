use rustsat::clause;
use rustsat::solvers::Solve;
use rustsat::types::{Clause, Lit, Var};
use rustsat_batsat::BasicSolver;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;

use crate::HitmanError;

// threshold for hitting sets for the
// function hit used when we need to
// filter duplication, different strategies
// with this constant
// TODO: test DEDUP_THRESHOLD
const DEDUP_THRESHOLD: usize = 100;

pub struct Hitman {
    oracle: BasicSolver,
    id2var: HashMap<u32, Var>,
    var2id: Vec<u32>,
    unsat: bool,
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
            oracle: new_minimal_solver(),
            var2id: Vec::new(),
            id2var: HashMap::new(),
            unsat: false,
        }
    }

    /// Add a new set to hit
    /// # Errors
    /// Returns [`HitmanError`]
    /// if more than ``Var::MAX_IDX`` distinct elements are added
    pub fn hit(&mut self, set: &[u32]) -> Result<(), HitmanError> {
        self.add_clause(set.iter().map(|id| (*id, true)))?;
        Ok(())
    }

    /// Add the set that is not going to be report
    /// as hitting set
    /// # Errors
    /// Returns [`HitmanError`]
    /// if more than ``Var::MAX_IDX`` distinct elements are added
    pub fn block(&mut self, set: &[u32]) -> Result<(), HitmanError> {
        self.add_clause(set.iter().map(|id| (*id, false)))?;
        Ok(())
    }

    fn add_clause(
        &mut self,
        lits: impl IntoIterator<Item = (u32, bool)>,
    ) -> Result<(), HitmanError> {
        let clause = lits
            .into_iter()
            .map(|(id, positive)| {
                let var = self.get_or_create_var(id)?;

                // The arg of lit is negated
                Ok(var.lit(!positive))
            })
            .collect::<Result<Clause, HitmanError>>()?;

        if clause.is_empty() {
            self.unsat = true;
        }

        self.oracle
            .add_clause(clause)
            .map_err(|e| HitmanError::Solver(e.to_string()))?;
        Ok(())
    }

    /// Add a new set to hit
    /// if it has duplicates, it filters them
    /// # Errors
    /// Returns [`HitmanError`] if the number of
    /// lits is greater than u32 type
    pub fn hit_filtering(&mut self, set: &[u32]) -> Result<(), HitmanError> {
        self.add_clause_filtering(set.iter().map(|id| (*id, true)))?;
        Ok(())
    }

    fn add_clause_filtering(
        &mut self,
        lits: impl IntoIterator<Item = (u32, bool)>,
    ) -> Result<(), HitmanError> {
        let lits_iter = lits.into_iter();

        let (lower, _) = lits_iter.size_hint();
        let clause = if lower > DEDUP_THRESHOLD {
            let mut current_clause: Vec<Lit> = Vec::with_capacity(lower);
            let mut seen = HashSet::with_capacity(lower);

            for (id, positive) in lits_iter {
                if seen.insert((id, positive)) {
                    let var = self.get_or_create_var(id)?;
                    current_clause.push(var.lit(!positive));
                }
            }
            current_clause
        } else {
            let mut current_clause: Vec<Lit> = Vec::with_capacity(lower);

            for (id, positive) in lits_iter {
                let var = self.get_or_create_var(id)?;
                let lit = var.lit(!positive);
                if !current_clause.contains(&lit) {
                    current_clause.push(lit);
                }
            }
            current_clause
        };

        if clause.is_empty() {
            self.unsat = true;
        }

        self.oracle
            .add_clause(Clause::from_iter(clause))
            .map_err(|e| HitmanError::Solver(e.to_string()))?;
        Ok(())
    }

    fn get_or_create_var(&mut self, id: u32) -> Result<Var, HitmanError> {
        let var_entry = self.id2var.entry(id);
        match var_entry {
            Entry::Occupied(occupied) => Ok(*occupied.get()),
            Entry::Vacant(vacant) => {
                let size_ids = self.var2id.len();
                let new_id = u32::try_from(size_ids)
                    .map_err(|_| HitmanError::TooManyElements { value: size_ids })?;
                if new_id > Var::MAX_IDX {
                    return Err(HitmanError::TooManyElements { value: size_ids });
                }
                let var = Var::new(new_id);
                self.var2id.push(id);
                vacant.insert(var);
                Ok(var)
            }
        }
    }

}

fn new_minimal_solver() -> BasicSolver {
    let mut solver = BasicSolver::default();
    let opts = batsat::SolverOpts {
        phase_saving: 0,
        ..Default::default()
    };
    *solver.batsat_mut() = batsat::Solver::new(opts, batsat::BasicCallbacks::default());
    solver
}

fn is_minimal_hitting_set(
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

fn is_subset(sub: &[u32], set: &HashSet<u32>) -> bool {
    for elem in sub {
        if !set.contains(elem) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_empty() {
        let hitman = Hitman::new();
        assert_eq!(hitman.id2var.len(), 0);
        assert!(!hitman.unsat);
    }

    #[test]
    fn hit_assigns_ids() {
        let mut hitman = Hitman::new();
        hitman.hit(&[28, 10]).unwrap();
        hitman.hit(&[28, 11]).unwrap();
        hitman.hit(&[29, 28, 30, 10]).unwrap();
        assert_eq!(hitman.var2id.first(), Some(&28u32));
        assert_eq!(hitman.var2id.get(1), Some(&10u32));
        assert_eq!(hitman.var2id.get(2), Some(&11u32));
        assert_eq!(hitman.var2id.get(3), Some(&29u32));
        assert_eq!(hitman.var2id.get(4), Some(&30u32));
        assert_eq!(hitman.id2var.len(), 5);
    }

    #[test]
    fn empty_clause() {
        let mut hitman = Hitman::new();
        hitman.hit(&[]).unwrap();
        assert_eq!(hitman.id2var.len(), 0);
        assert!(hitman.unsat);
    }

    #[test]
    fn id_round_trip() {
        let mut hitman = Hitman::new();
        hitman.hit(&[28, 10]).unwrap();
        hitman.hit(&[28, 11]).unwrap();
        hitman.hit(&[29, 28, 30, 10]).unwrap();
        assert_eq!(hitman.var2id, [28, 10, 11, 29, 30]);
    }
}
