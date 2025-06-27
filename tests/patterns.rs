use auto_hash::expand_variable_patterns;

#[test]
fn expand_question_x_varies_with_min_max() {
    let patterns = expand_variable_patterns("pre?xpost", 1, 3);
    assert_eq!(patterns, vec![
        "pre?1post",
        "pre?1?1post",
        "pre?1?1?1post",
    ]);
}

#[test]
fn returns_original_when_placeholder_absent() {
    let patterns = expand_variable_patterns("no_placeholder", 2, 4);
    assert_eq!(patterns, vec!["no_placeholder"]);
}
