use std::fmt;
#[derive(Debug)]
pub enum HitmanError {
    OutOfRange { value: usize, max: u32 },
}

impl fmt::Display for HitmanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HitmanError::OutOfRange { value, max } => {
                write!(f, "{value} is above the max {max}")
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
            max: max_u32,
        };
        assert_eq!(
            err_out_of_range.to_string(),
            format!("{max_usize} is above the max {max_u32}")
        );
    }
}
