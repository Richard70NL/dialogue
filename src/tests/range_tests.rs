/************************************************************************************************/

use crate::data::Range;

/************************************************************************************************/

#[test]
fn empty_test() {
    let range = Range::parse("").unwrap();
    assert_eq!(range.from, 0);
    assert_eq!(range.to, std::i32::MAX);
}

/************************************************************************************************/

#[test]
fn produce_error() {
    match Range::parse("this_will_produce-an_error") {
        Ok(_) => assert!(false, "an error should have been produces"),
        Err(_) => assert!(true, "an error should have been produced"),
    }
}

/************************************************************************************************/

#[test]
fn from_only() {
    let range = Range::parse("5-").unwrap();
    assert_eq!(range.from, 5);
    assert_eq!(range.to, std::i32::MAX);
}

/************************************************************************************************/

#[test]
fn to_only() {
    let range = Range::parse("-99").unwrap();
    assert_eq!(range.from, 0);
    assert_eq!(range.to, 99);
}

/************************************************************************************************/

#[test]
fn from_and_to() {
    let range = Range::parse("27-44").unwrap();
    assert_eq!(range.from, 27);
    assert_eq!(range.to, 44);
}

/************************************************************************************************/
