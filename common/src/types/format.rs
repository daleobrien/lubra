/// Trims trailing zeros and decimal points from f64, and strips leading and trailing spaces.
///
/// # Arguments
/// * `num` - The f64 number to format
///
/// # Returns
/// A String with trailing zeros and decimal points removed, and no leading/trailing spaces
///
/// # Examples
/// ```
/// use common::types::format::trimf64;
///
/// assert_eq!(trimf64(1.0), "1");
/// assert_eq!(trimf64(1.5), "1.5");
/// assert_eq!(trimf64(-2.30), "-2.3");
/// ```
pub fn trimf64(num: f64) -> String {
    let formatted = format!("{:.*}", 1, num);
    let trimmed = formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .trim();
    return trimmed.to_string();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
    fn test_trimf64_strip_positive_integer() {
        assert_eq!(trimf64(1.0), "1");
    }

    #[test]
    fn test_trimf64_strip_positive_decimal() {
        assert_eq!(trimf64(1.5), "1.5");
    }

    #[test]
    fn test_trimf64_strip_positive_trailing_zeros() {
        assert_eq!(trimf64(2.30), "2.3");
    }

    #[test]
    fn test_trimf64_strip_negative_integer() {
        assert_eq!(trimf64(-1.0), "-1");
    }

    #[test]
    fn test_trimf64_strip_negative_decimal() {
        assert_eq!(trimf64(-1.5), "-1.5");
    }

    #[test]
    fn test_trimf64_strip_negative_trailing_zeros() {
        assert_eq!(trimf64(-2.30), "-2.3");
    }

    #[test]
    fn test_trimf64_strip_zero() {
        assert_eq!(trimf64(0.0), "0");
    }

    #[test]
    fn test_trimf64_strip_no_leading_space() {
        // Verify that positive numbers don't have leading space
        let result = trimf64(5.0);
        assert!(!result.starts_with(' '));
        assert_eq!(result, "5");
    }
}
