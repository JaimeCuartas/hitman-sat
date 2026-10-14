use rustsat::solvers::{Solve, SolverResult};
use rustsat::types::Clause;
use rustsat::{clause, lit};
use rustsat_cadical::CaDiCaL;

/// x0 \/ x1 nd not x0
pub fn small_sat() -> bool {
    let mut s = CaDiCaL::default();
    let mut c1 = Clause::new();
    c1.add(lit![0]);
    c1.add(lit![1]);
    let _ = s.add_clause(c1);
    let _ = s.add_clause(clause![!lit![0]]);
    s.solve().unwrap() == SolverResult::Sat
}

#[cfg(test)]
mod tests {
    use rustsat::{solvers::{Solve, SolverResult}, types::TernaryVal};
    use rustsat_cadical::CaDiCaL;
    use rustsat::{clause, lit};

    #[test]
    fn sat_case_finds_correct_model() {
        let mut s = CaDiCaL::default();
        s.add_clause(clause![lit![0], lit![1]]).unwrap();
        s.add_clause(clause![!lit![0]]).unwrap();
        let solution = s.solve().unwrap();
        assert!(solution == SolverResult::Sat);
        assert_eq!(s.lit_val(lit![0]).unwrap(), TernaryVal::False);
        assert_eq!(s.lit_val(lit![1]).unwrap(), TernaryVal::True);
    }
    #[test]
    fn unsat_case() {
        let mut s = CaDiCaL::default();
        let _ = s.add_clause(clause![lit![0], lit![1]]);
        let _ = s.add_clause(clause![!lit![0]]);
        let _ = s.add_clause(clause![!lit![1]]);
        assert_eq!(s.solve().unwrap(), SolverResult::Unsat);
    }
}
