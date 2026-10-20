#[derive(Debug, PartialEq, thiserror::Error)]
pub enum HitmanError {

    #[error("The number of ids is {value}, which is above the max {max}", max = u32::MAX)]
    TooManyElements { value: usize },

    #[error("Solver Error: {0}")]
    Solver(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_message() {
        let max_u32 = u32::MAX;
        let err_out_of_range = HitmanError::TooManyElements { value: 28 };
        assert_eq!(
            err_out_of_range.to_string(),
            format!("The number of ids is 28, which is above the max {max_u32}")
        );
    }
}
