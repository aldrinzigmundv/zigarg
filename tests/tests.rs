#[cfg(test)]

use zigarg::*;

#[test]
fn has_args() {
    let args = Arguments(vec!["onearg".to_string()]);
    assert_eq!(args.has_args(), false);
    let args2 = Arguments(vec!["onearg".to_string(), "twoargs".to_string()]);
    assert_eq!(args2.has_args(), true);
    }

#[test]
fn len() {
    let args = Arguments(vec!["onearg".to_string()]);
    assert_eq!(args.len(), 1);
    let args2 = Arguments(vec!["onearg".to_string(), "twoargs".to_string()]);
    assert_eq!(args2.len(), 2);
}

#[test]
fn eq() {
    let args = Arguments(vec!["onearg".to_string()]);
    assert_eq!(args.eq(0), false);
    assert_eq!(args.eq(1), true);
    assert_eq!(args.eq(2), false);
}

#[test]
fn get() {
    let args = Arguments(vec!["onearg".to_string()]);
    assert_eq!(args.get(0).eq_ignore_ascii_case(&"onearg"), true);
    assert_eq!(args.get(0).eq_ignore_ascii_case(&"twoarg"), false);
    let args2 = Arguments(vec!["onearg".to_string(), "twoargs".to_string()]);
    assert_eq!(args2.get(1).eq_ignore_ascii_case(&"onearg"), false);
    assert_eq!(args2.get(1).eq_ignore_ascii_case(&"twoargs"), true);
}

#[test]
fn exist() {
    let args = Arguments(vec!["onearg".to_string(), "twoargs".to_string()]);
    assert_eq!(args.exist("Onearg"), true);
    assert_eq!(args.exist("TwoArgs"), true);
    assert_eq!(args.exist("threeArgs"), false);
}

#[test]
fn exist_case_sensitive() {
    let args = Arguments(vec!["onearg".to_string(), "twoargs".to_string()]);
    assert_eq!(args.exist_case_sensitive("Onearg"), false);
    assert_eq!(args.exist_case_sensitive("twoargs"), true);
    assert_eq!(args.exist_case_sensitive("threeArgs"), false);
}