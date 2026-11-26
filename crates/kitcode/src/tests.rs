use super::run;

#[test]
fn prints_version_and_help() {
    assert_eq!(run(&["--version".to_string()]), 0);
    assert_eq!(run(&["--help".to_string()]), 0);
    assert_eq!(run(&[]), 0);
    assert_eq!(run(&["--unknown".to_string()]), 2);
}
