use std::{error, fmt};
use error::Error;
use std::num::ParseIntError;

pub struct Range {
    pub start: u64,
    pub end: Option<u64>,
}

pub struct MalformedRangeError {
    pub message: String
}


#[derive(Debug)]
pub enum RangeParseError {
    RangeEndParseError(ParseIntError),
    MalformedRangeError(String),
}

impl From<ParseIntError> for RangeParseError {
    fn from(err: ParseIntError) -> RangeParseError {
        RangeParseError::RangeEndParseError(err)
    }
}

impl Error for RangeParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            RangeParseError::RangeEndParseError(ref e) => { Some(e) }
            RangeParseError::MalformedRangeError(ref _s) => { None }
        }
    }
}

impl fmt::Display for RangeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RangeParseError::RangeEndParseError(ref e) => e.fmt(f),
            RangeParseError::MalformedRangeError(ref s) => {
                write!(f, "{}", *s)
            }
        }
    }
}


impl Range {
    pub fn parse_range(range_header: &String) -> Result<Range, RangeParseError> {
        let interval = range_header.replace("bytes=", "");
        let divider = interval.find("-");

        return match divider {
            Some(pos) => {
                let start = interval[0..pos].parse::<u64>()?;
                let end =
                    if interval.chars().count() == pos + 1 {
                        None
                    } else {
                        Some(interval[pos + 1..interval.len()].parse::<u64>()?)
                    };
                Ok(Range { start, end })
            }
            None => {
                Result::Err(RangeParseError::MalformedRangeError { 0: range_header.clone() })
            }
        };
    }
}
