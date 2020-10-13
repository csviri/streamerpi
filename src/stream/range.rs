pub struct Range {
    pub start: u64,
    pub end: Option<u64>,
}

impl Range {
    pub fn parse_range(range_header: String) -> Result<Range, String> {
        let interval = range_header.replace("bytes=", "");
        let divider = interval.find("-");

        return match divider {
            Some(pos) => {
                //todo this can kill the server, improve
                let start = interval[0..pos].parse::<u64>().unwrap();
                let end =
                    if interval.chars().count() == pos + 1 {
                        None
                    } else {
                        //todo this can kill the server, improve
                        Some(interval[pos+1..interval.len()].parse::<u64>().unwrap())
                    };
                Ok(Range { start, end })
            }
            None => {
                Err(format!("Invalid range header: {}", range_header))
            }
        };
    }
}
