/// Extracts branch visit history from checkout history.
/// Returns: [current branch, 1 step back, 2 steps back, ...]
pub fn extract_branch_history(reflog: &str) -> Vec<String> {
    let mut history = Vec::new();

    for line in reflog.lines() {
        if let Some(pos) = line.find("moving from ") {
            let rest = &line[pos + "moving from ".len()..];
            if let Some(to_pos) = rest.find(" to ") {
                let from_branch = &rest[..to_pos];
                let to_branch = &rest[to_pos + " to ".len()..];

                // If this is the first checkout, 'to' is the current branch
                if history.is_empty() {
                    history.push(to_branch.to_string());
                }
                // 'from' is the branch we were on before
                history.push(from_branch.to_string());
            }
        }
    }

    history
}

/// Gets the branch N steps back
pub fn get_nth_previous_branch(history: &[String], n: usize) -> Option<&String> {
    history.get(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_checkout_history() {
        let reflog = r#"
9c346b78 HEAD@{0}: commit: fix typo
9c346b78 HEAD@{1}: checkout: moving from feature/fix_board to feature/fix_rake
e5f6g7h8 HEAD@{2}: checkout: moving from main to feature/fix_board
m3n4o5p6 HEAD@{3}: checkout: moving from develop to main
"#;
        let history = extract_branch_history(reflog);

        assert_eq!(history.len(), 4);
        assert_eq!(history[0], "feature/fix_rake"); // current
        assert_eq!(history[1], "feature/fix_board"); // 1 step back
        assert_eq!(history[2], "main"); // 2 steps back
        assert_eq!(history[3], "develop"); // 3 steps back
    }

    #[test]
    fn test_single_checkout() {
        let reflog = "abc123 HEAD@{0}: checkout: moving from main to feature/new";
        let history = extract_branch_history(reflog);

        assert_eq!(history.len(), 2);
        assert_eq!(history[0], "feature/new");
        assert_eq!(history[1], "main");
    }

    #[test]
    fn test_no_checkout_history() {
        let reflog = r#"
abc123 HEAD@{0}: commit: add feature
def456 HEAD@{1}: commit: initial commit
"#;
        let history = extract_branch_history(reflog);
        assert!(history.is_empty());
    }

    #[test]
    fn test_branch_with_slashes() {
        let reflog = r#"
abc123 HEAD@{0}: checkout: moving from feature/2024/12/24_fix to release/v1.0.0
def456 HEAD@{1}: checkout: moving from hotfix/urgent-fix to feature/2024/12/24_fix
"#;
        let history = extract_branch_history(reflog);

        assert_eq!(history.len(), 3);
        assert_eq!(history[0], "release/v1.0.0");
        assert_eq!(history[1], "feature/2024/12/24_fix");
        assert_eq!(history[2], "hotfix/urgent-fix");
    }

    #[test]
    fn test_same_branch_multiple_times() {
        let reflog = r#"
abc123 HEAD@{0}: checkout: moving from main to feature/a
def456 HEAD@{1}: checkout: moving from feature/a to main
ghi789 HEAD@{2}: checkout: moving from main to feature/a
"#;
        let history = extract_branch_history(reflog);

        assert_eq!(history.len(), 4);
        assert_eq!(history[0], "feature/a");
        assert_eq!(history[1], "main");
        assert_eq!(history[2], "feature/a");
        assert_eq!(history[3], "main");
    }

    #[test]
    fn test_get_nth_previous_branch() {
        let history = vec![
            "current".to_string(),
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
        ];

        assert_eq!(get_nth_previous_branch(&history, 0), Some(&"current".to_string()));
        assert_eq!(get_nth_previous_branch(&history, 1), Some(&"first".to_string()));
        assert_eq!(get_nth_previous_branch(&history, 3), Some(&"third".to_string()));
        assert_eq!(get_nth_previous_branch(&history, 4), None);
    }

    #[test]
    fn test_empty_history() {
        let history: Vec<String> = vec![];
        assert_eq!(get_nth_previous_branch(&history, 0), None);
    }

    #[test]
    fn test_mixed_reflog_entries() {
        let reflog = r#"
abc123 HEAD@{0}: commit: fix bug
def456 HEAD@{1}: pull: Fast-forward
ghi789 HEAD@{2}: checkout: moving from develop to feature/test
jkl012 HEAD@{3}: merge feature/other: Merge made by recursive
mno345 HEAD@{4}: checkout: moving from main to develop
pqr678 HEAD@{5}: rebase finished: refs/heads/main onto abc123
stu901 HEAD@{6}: checkout: moving from feature/old to main
"#;
        let history = extract_branch_history(reflog);

        assert_eq!(history.len(), 4);
        assert_eq!(history[0], "feature/test");
        assert_eq!(history[1], "develop");
        assert_eq!(history[2], "main");
        assert_eq!(history[3], "feature/old");
    }

    #[test]
    fn test_branch_with_special_characters() {
        let reflog = r#"
abc123 HEAD@{0}: checkout: moving from feature/fix_bug-123 to release-v2.0
def456 HEAD@{1}: checkout: moving from bugfix/issue#456 to feature/fix_bug-123
"#;
        let history = extract_branch_history(reflog);

        assert_eq!(history.len(), 3);
        assert_eq!(history[0], "release-v2.0");
        assert_eq!(history[1], "feature/fix_bug-123");
        assert_eq!(history[2], "bugfix/issue#456");
    }
}
