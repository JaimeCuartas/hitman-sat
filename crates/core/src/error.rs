use std::fmt;
#[derive(Debug, PartialEq)]
pub enum HitmanError {
    OutOfRange { value: usize },
}

impl fmt::Display for HitmanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HitmanError::OutOfRange { value } => {
                write!(f, "The number of ids is {value}, which is above the max {}", u32::MAX)
            }
        }
    }
}

impl std::error::Error for HitmanError {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_message() {
        let max_u32 = u32::MAX;
        let max_usize = usize::MAX;
        let err_out_of_range = HitmanError::OutOfRange {
            value: max_usize,
        };
        assert_eq!(
            err_out_of_range,
            HitmanError::OutOfRange { value: max_usize }
        );
        assert_eq!(
            err_out_of_range.to_string(),
            format!("The number of ids is {max_usize}, which is above the max {max_u32}")
        );
    }
}
