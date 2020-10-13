use crate::stream::range::Range;

#[test]
fn range_with_open_end() {
    let res: Range = Range::parse_range(String::from("bytes=0-")).unwrap();
    assert_eq!(0, res.start);
    assert!(res.end.is_none());

    let res = Range::parse_range(String::from("bytes=123-")).unwrap();
    assert_eq!(123, res.start);
    assert!(res.end.is_none());
}

#[test]
fn range_with_close_end() {
    let res = Range::parse_range(String::from("bytes=0-10")).unwrap();
    assert_eq!(0, res.start);
    assert_eq!(10, res.end.expect("No value"));

    let res = Range::parse_range(String::from("bytes=123-500")).unwrap();
    assert_eq!(123, res.start);
    assert_eq!(500, res.end.expect("No value"));
}