use rustsat::types::Var;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum HitmanError {
    #[error("The number of ids is {value}, which is above the max {max}", max = Var::MAX_IDX)]
    TooManyElements { value: usize },

    #[error("Solver Error: {0}")]
    Solver(String),
}

#[cfg(test)]
mod tests {
    use rustsat::types::Var;

    use super::*;
    #[test]
    fn display_message() {
        let max_var_value = Var::MAX_IDX;
        let err_out_of_range = HitmanError::TooManyElements { value: 28 };
        assert_eq!(
            err_out_of_range.to_string(),
            format!("The number of ids is 28, which is above the max {max_var_value}")
        );
    }
}
