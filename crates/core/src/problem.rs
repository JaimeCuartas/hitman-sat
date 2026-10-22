use batsat::SolverInterface;
use rustsat::solvers::{Solve, SolverResult};
use rustsat::types::{Clause, Lit, TernaryVal, Var};
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

    /// Retreive a hitting set
    /// # Errors
    /// Returns [`HitmanError`]
    /// if SAT solver is interrupted
    pub fn get(&mut self) -> Result<Option<Vec<u32>>, HitmanError> {
        if self.unsat {
            return Ok(None);
        }

        let is_sat = self
            .oracle
            .solve()
            .map_err(|e| HitmanError::Solver(e.to_string()))?;

        match is_sat {
            SolverResult::Sat => {
                let mut hitting_set = Vec::new();

                for (i, original_id) in (0u32..).zip(&self.var2id) {
                    let var = Var::new(i);
                    let var_value = self
                        .oracle
                        .var_val(var)
                        .map_err(|e| HitmanError::Solver(e.to_string()))?;

                    if var_value == TernaryVal::True {
                        hitting_set.push(*original_id);
                    }
                }
                Ok(Some(hitting_set))
            }
            SolverResult::Unsat => {
                self.unsat = true;
                Ok(None)
            }
            SolverResult::Interrupted => Err(HitmanError::Interrupted),
        }
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
                register_var_negative_polarity(&mut self.oracle, var);
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
        rnd_pol: false,
        ..Default::default()
    };
    *solver.batsat_mut() = batsat::Solver::new(opts, batsat::BasicCallbacks::default());
    solver
}

fn register_var_negative_polarity(oracle: &mut BasicSolver, var: Var) {
    let batsat_var = oracle.batsat_mut().new_var(batsat::lbool::FALSE, true);
    debug_assert_eq!(
        batsat_var.idx(),
        var.idx32(),
        "BatSat and rustsat var indices must match"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn is_minimal_hitting_set_correct() {
        let collection_sets = vec![vec![1, 2], vec![2, 3], vec![3]];
        assert!(!is_minimal_hitting_set(&[1, 2, 3], &collection_sets, &[]));
        assert!(!is_minimal_hitting_set(&[1, 2], &collection_sets, &[]));
        assert!(is_minimal_hitting_set(&[1, 3], &collection_sets, &[]));
        // It is not because it should be blocked
        assert!(!is_minimal_hitting_set(
            &[1, 3],
            &collection_sets,
            &[vec![3, 1]]
        ));
        assert!(is_minimal_hitting_set(
            &[2, 3],
            &collection_sets,
            &[vec![3, 1]]
        ));
    }

    fn is_subset(sub: &[u32], set: &HashSet<u32>) -> bool {
        for elem in sub {
            if !set.contains(elem) {
                return false;
            }
        }
        true
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

    fn is_same_collection_of_set(a: &[Vec<u32>], b: &[Vec<u32>]) -> bool {
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

    #[test]
    fn new_is_empty() {
        let hitman = Hitman::new();
        assert_eq!(hitman.id2var.len(), 0);
        assert!(!hitman.unsat);
    }

    #[test]
    fn get_empty_hitting_sets() {
        let mut hitman = Hitman::new();
        hitman.hit(&[]).unwrap();
        assert_eq!(hitman.get().unwrap(), None);
    }
    #[test]
    fn get_empty_hitting_set_adding_set_after() {
        let mut hitman = Hitman::new();
        hitman.hit(&[]).unwrap();
        hitman.hit(&[1, 2]).unwrap();
        assert_eq!(hitman.get().unwrap(), None);
    }

    #[test]
    fn get_empty_hitting_set_adding_set_before() {
        let mut hitman = Hitman::new();
        hitman.hit(&[1, 2]).unwrap();
        hitman.hit(&[]).unwrap();
        assert_eq!(hitman.get().unwrap(), None);
    }

    #[test]
    fn get_empty_hitting_set_w_no_hits() {
        let mut hitman = Hitman::new();
        assert_eq!(hitman.get().unwrap(), Some(vec![]));
    }

    #[test]
    fn get_unique_hitting_set() {
        let mut hitman = Hitman::new();
        hitman.hit(&[1]).unwrap();
        assert_eq!(hitman.get().unwrap(), Some(vec![1]));
        hitman.block(&[1]).unwrap();
        assert_eq!(hitman.get().unwrap(), None);
    }

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
