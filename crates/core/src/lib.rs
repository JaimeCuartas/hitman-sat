///
use rustsat::lit;
use rustsat::solvers::{Solve, SolverResult};
use rustsat::types::Clause;
use rustsat_cadical::CaDiCaL;

/// x0 \/ x1 nd not x0
pub fn small_sat() -> bool {
    let mut s = CaDiCaL::default();
    let mut c1 = Clause::new();
    c1.add(lit![0]);
    c1.add(lit![1]);
    s.solve().unwrap() == SolverResult::Sat
}

#[cfg(test)]
mod tests {
    #[test]
    fn cadical_works() {
        let result = super::small_sat();
        assert!(result);
    }
}
