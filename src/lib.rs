/// Library for auto-hash utilities.

/// Expand patterns with variable '?x' into sequences of '?1' repeated
/// between `min` and `max` times. If the pattern doesn't contain `?x`,
/// the original pattern is returned.
pub fn expand_variable_patterns(base_pattern: &str, min: usize, max: usize) -> Vec<String> {
    let mut patterns = Vec::new();
    if base_pattern.contains("?x") {
        for len in min..=max {
            let replacement = "?1".repeat(len);
            let pat = base_pattern.replace("?x", &replacement);
            patterns.push(pat);
        }
    } else {
        patterns.push(base_pattern.to_string());
    }
    patterns
}

#[cfg(test)]
mod tests {
    use super::expand_variable_patterns;

    #[test]
    fn test_expand_var() {
        let result = expand_variable_patterns("ab?xcd", 1, 3);
        assert_eq!(result, vec!["ab?1cd", "ab?1?1cd", "ab?1?1?1cd"]);
    }

    #[test]
    fn test_without_placeholder() {
        let result = expand_variable_patterns("abc", 1, 2);
        assert_eq!(result, vec!["abc"]);
    }
}

