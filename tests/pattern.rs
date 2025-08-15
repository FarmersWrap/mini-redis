use mini_redis::Pattern;

#[test]
fn test_pattern_exact_match() {
    let pattern = Pattern::new("exact").unwrap();

    assert!(pattern.matches("exact"));
    assert!(!pattern.matches("exactly"));
    assert!(!pattern.matches("inexact"));
    assert!(!pattern.matches(""));
}

#[test]
fn test_pattern_wildcard_star() {
    let pattern = Pattern::new("*").unwrap();

    assert!(pattern.matches(""));
    assert!(pattern.matches("a"));
    assert!(pattern.matches("hello"));
    assert!(pattern.matches("very long string with spaces"));
}

#[test]
fn test_pattern_wildcard_question() {
    let pattern = Pattern::new("?").unwrap();

    assert!(!pattern.matches(""));
    assert!(pattern.matches("a"));
    assert!(pattern.matches("b"));
    assert!(!pattern.matches("ab"));
    assert!(!pattern.matches("hello"));
}

#[test]
fn test_pattern_star_prefix() {
    let pattern = Pattern::new("*test").unwrap();

    assert!(pattern.matches("test"));
    assert!(pattern.matches("atest"));
    assert!(pattern.matches("hello test"));
    assert!(pattern.matches("very long test"));
    assert!(!pattern.matches("test123"));
    assert!(!pattern.matches("testing"));
}

#[test]
fn test_pattern_star_suffix() {
    let pattern = Pattern::new("test*").unwrap();

    assert!(pattern.matches("test"));
    assert!(pattern.matches("test123"));
    assert!(pattern.matches("testing"));
    assert!(pattern.matches("test with spaces"));
    assert!(!pattern.matches("atest"));
    assert!(!pattern.matches("hello test"));
}

#[test]
fn test_pattern_star_middle() {
    let pattern = Pattern::new("a*b").unwrap();

    assert!(pattern.matches("ab"));
    assert!(pattern.matches("aab"));
    assert!(pattern.matches("abb"));
    assert!(pattern.matches("a123b"));
    assert!(pattern.matches("a very long string b"));
    assert!(!pattern.matches("ac"));
    assert!(!pattern.matches("cb"));
    assert!(!pattern.matches(""));
}

#[test]
fn test_pattern_question_prefix() {
    let pattern = Pattern::new("?test").unwrap();

    assert!(!pattern.matches("test"));
    assert!(pattern.matches("atest"));
    assert!(pattern.matches("btest"));
    assert!(!pattern.matches("aatest"));
    assert!(!pattern.matches(""));
}

#[test]
fn test_pattern_question_suffix() {
    let pattern = Pattern::new("test?").unwrap();

    assert!(!pattern.matches("test"));
    assert!(pattern.matches("testa"));
    assert!(pattern.matches("test1"));
    assert!(!pattern.matches("testab"));
    assert!(!pattern.matches(""));
}

#[test]
fn test_pattern_question_middle() {
    let pattern = Pattern::new("a?b").unwrap();

    assert!(!pattern.matches("ab"));
    assert!(pattern.matches("aab"));
    assert!(pattern.matches("acb"));
    assert!(pattern.matches("a1b"));
    assert!(!pattern.matches("aabb"));
    assert!(!pattern.matches("ac"));
    assert!(!pattern.matches("cb"));
}

#[test]
fn test_pattern_multiple_wildcards() {
    let pattern = Pattern::new("a*b*c").unwrap();

    assert!(pattern.matches("abc"));
    assert!(pattern.matches("a1bc"));
    assert!(pattern.matches("ab1c"));
    assert!(pattern.matches("a1b2c"));
    assert!(pattern.matches("a very long string with b and c"));
    assert!(!pattern.matches("ac"));
    assert!(!pattern.matches("ab"));
    assert!(!pattern.matches("bc"));
}

#[test]
fn test_pattern_question_and_star() {
    let pattern = Pattern::new("a?b*").unwrap();

    assert!(!pattern.matches("ab"));
    assert!(pattern.matches("aab"));
    assert!(pattern.matches("aab123"));
    assert!(pattern.matches("aab with spaces"));
    assert!(!pattern.matches("abb"));
    assert!(!pattern.matches("ac"));
}

#[test]
fn test_pattern_complex_combinations() {
    let pattern = Pattern::new("user*?profile").unwrap();

    assert!(!pattern.matches("userprofile"));
    assert!(pattern.matches("user1profile"));
    assert!(pattern.matches("useraprofile"));
    assert!(pattern.matches("user123profile"));
    assert!(pattern.matches("user_name_profile"));
    assert!(!pattern.matches("userprofile123"));
    assert!(!pattern.matches("profile"));
}

#[test]
fn test_pattern_redis_style() {
    // Test patterns similar to Redis PSUBSCRIBE patterns

    // user:*:profile
    let pattern = Pattern::new("user:*:profile").unwrap();
    assert!(pattern.matches("user:123:profile"));
    assert!(pattern.matches("user:john:profile"));
    assert!(pattern.matches("user:very_long_name:profile"));
    assert!(!pattern.matches("user:profile"));
    assert!(!pattern.matches("user:123:profile:extra"));

    // news:*
    let pattern = Pattern::new("news:*").unwrap();
    assert!(pattern.matches("news:sports"));
    assert!(pattern.matches("news:technology"));
    assert!(pattern.matches("news:"));
    assert!(!pattern.matches("oldnews:sports"));

    // *:update
    let pattern = Pattern::new("*:update").unwrap();
    assert!(pattern.matches("user:update"));
    assert!(pattern.matches("system:update"));
    assert!(pattern.matches("very_long_name:update"));
    assert!(!pattern.matches("update"));
    assert!(!pattern.matches("user:update:extra"));
}

#[test]
fn test_pattern_edge_cases() {
    // Empty pattern
    let pattern = Pattern::new("").unwrap();
    assert!(pattern.matches(""));
    assert!(!pattern.matches("a"));

    // Pattern with only wildcards
    let pattern = Pattern::new("***").unwrap();
    assert!(pattern.matches(""));
    assert!(pattern.matches("a"));
    assert!(pattern.matches("hello"));

    let pattern = Pattern::new("???").unwrap();
    assert!(!pattern.matches(""));
    assert!(!pattern.matches("a"));
    assert!(!pattern.matches("ab"));
    assert!(pattern.matches("abc"));
    assert!(pattern.matches("123"));

    // Pattern with escaped characters (if supported)
    let pattern = Pattern::new("a\\*b").unwrap();
    // This would match literal "a*b" if escaping is supported
    // For now, assuming no escaping, so this matches "a" followed by any chars followed by "b"
    assert!(pattern.matches("a*b"));
    assert!(pattern.matches("a1b"));
    assert!(pattern.matches("a very long string b"));
}

#[test]
fn test_pattern_unicode() {
    let pattern = Pattern::new("user*世界").unwrap();

    assert!(pattern.matches("user世界"));
    assert!(pattern.matches("user123世界"));
    assert!(pattern.matches("user_name世界"));
    assert!(!pattern.matches("世界"));
    assert!(!pattern.matches("user世界extra"));

    let pattern = Pattern::new("こんにちは*").unwrap();

    assert!(pattern.matches("こんにちは"));
    assert!(pattern.matches("こんにちは世界"));
    assert!(pattern.matches("こんにちは123"));
    assert!(!pattern.matches("こんにち"));
    assert!(!pattern.matches("extraこんにちは"));
}

#[test]
fn test_pattern_case_sensitivity() {
    let pattern = Pattern::new("User*").unwrap();

    assert!(pattern.matches("User123"));
    assert!(pattern.matches("Username"));
    assert!(!pattern.matches("user123"));
    assert!(!pattern.matches("USER123"));

    let pattern = Pattern::new("user*").unwrap();

    assert!(pattern.matches("user123"));
    assert!(pattern.matches("username"));
    assert!(!pattern.matches("User123"));
    assert!(!pattern.matches("USER123"));
}

#[test]
fn test_pattern_special_characters() {
    let pattern = Pattern::new("user*@domain.com").unwrap();

    assert!(pattern.matches("user@domain.com"));
    assert!(pattern.matches("user123@domain.com"));
    assert!(pattern.matches("user_name@domain.com"));
    assert!(!pattern.matches("@domain.com"));
    assert!(!pattern.matches("user@domain.com.extra"));

    let pattern = Pattern::new("file*.txt").unwrap();

    assert!(pattern.matches("file.txt"));
    assert!(pattern.matches("file123.txt"));
    assert!(pattern.matches("my_file.txt"));
    assert!(!pattern.matches("file.txt.old"));
    assert!(!pattern.matches(".txt"));
}

#[test]
fn test_pattern_validation() {
    // Test invalid patterns
    assert!(Pattern::new("").is_ok());
    assert!(Pattern::new("normal").is_ok());
    assert!(Pattern::new("*").is_ok());
    assert!(Pattern::new("?").is_ok());
    assert!(Pattern::new("a*b?c").is_ok());

    // If there are any validation rules, test them here
    // For now, assuming all patterns are valid
}

#[test]
fn test_pattern_performance() {
    // Test with very long patterns and strings
    let long_pattern = "a".repeat(1000) + "*" + &"b".repeat(1000);
    let pattern = Pattern::new(&long_pattern).unwrap();

    let long_string = "a".repeat(500) + "middle" + &"b".repeat(500);
    assert!(pattern.matches(&long_string));

    let short_string = "a".to_owned() + "b";
    assert!(pattern.matches(&short_string));
}

#[test]
fn test_pattern_redis_channels() {
    // Test common Redis channel patterns

    // All channels
    let pattern = Pattern::new("*").unwrap();
    assert!(pattern.matches("user:123:profile"));
    assert!(pattern.matches("system:logs"));
    assert!(pattern.matches("news:sports"));

    // User channels
    let pattern = Pattern::new("user:*").unwrap();
    assert!(pattern.matches("user:123:profile"));
    assert!(pattern.matches("user:john:settings"));
    assert!(!pattern.matches("system:logs"));

    // Specific user channels
    let pattern = Pattern::new("user:123:*").unwrap();
    assert!(pattern.matches("user:123:profile"));
    assert!(pattern.matches("user:123:settings"));
    assert!(!pattern.matches("user:456:profile"));
    assert!(!pattern.matches("system:logs"));

    // System channels
    let pattern = Pattern::new("system:*").unwrap();
    assert!(pattern.matches("system:logs"));
    assert!(pattern.matches("system:status"));
    assert!(!pattern.matches("user:123:profile"));
}
