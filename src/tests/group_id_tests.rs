/************************************************************************************************/

use crate::data::GroupId;

/************************************************************************************************/

#[test]
fn empty_test() {
    let gid = GroupId::from("");
    assert_eq!("", gid.to_string());
    assert!(gid.is_empty());
}

/************************************************************************************************/

#[test]
fn non_empty_test() {
    let gid = GroupId::from("some.group");
    assert_eq!("some.group", gid.to_string());
    assert!(!gid.is_empty());
}

/************************************************************************************************/
