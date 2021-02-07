use crate::stream::range::{Range, RangeParseError};

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

#[test]
fn malformed_range_error() {
    const FAKE_ERROR: &str = "error_range";

    let res = Range::parse_range(String::from(FAKE_ERROR));

    assert!(res.is_err());
    match res.err() {
        Some(range_parse_error) => {
            match range_parse_error {
                RangeParseError::MalformedRangeError(s) => {
                    assert_eq!(FAKE_ERROR, s)
                }
                _ => { assert!(false) }
            }
        }
        None => { assert!(false) }
    }
}

#[test]
fn cannot_parse_range_end() {
    const FAKE_ERROR: &str = "bytes=xxx-";

    let res = Range::parse_range(String::from(FAKE_ERROR));

    assert!(res.is_err());
    match res.err() {
        Some(range_parse_error) => {
            match range_parse_error {
                RangeParseError::RangeEndParseError(s) => {
                    println!("{}", s)
                }
                _ => { assert!(false) }
            }
        }
        None => { assert!(false) }
    }
}