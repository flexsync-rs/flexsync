use flexsync::erase;

#[test]
fn fn_with_duplicate_name() {}

#[erase]
#[test]
fn fn_with_duplicate_name() {}
