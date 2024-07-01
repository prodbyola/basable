use std::env;

pub(crate) fn get_env(key: &str) -> String {
    env::var(key).unwrap()
}

pub(crate) mod datetime_parser {
    use chrono::NaiveDateTime;

    pub(crate) struct DateFormat<'a>(pub &'a str, pub &'a str);
    
    impl<'a> DateFormat<'a> {
        pub fn patterns() -> [DateFormat<'a>; 41] {
            [
                DateFormat("%Y-%m-%d %H:%M:%S", "2024-07-01 14:30:45"),
                DateFormat("%d-%m-%Y %H:%M:%S", "01-07-2024 14:30:45"),
                DateFormat("%m-%d-%Y %H:%M:%S", "07-01-2024 14:30:45"),
                DateFormat("%d/%m/%Y %H:%M:%S", "01/07/2024 14:30:45"),
                DateFormat("%m/%d/%Y %H:%M:%S", "07/01/2024 14:30:45"),
                DateFormat("%d.%m.%Y %H:%M:%S", "01.07.2024 14:30:45"),
                DateFormat("%m.%d.%Y %H:%M:%S", "07.01.2024 14:30:45"),
                DateFormat("%Y.%m.%d %H:%M:%S", "2024.07.01 14:30:45"),
                DateFormat("%Y/%m/%d %H:%M:%S", "2024/07/01 14:30:45"),
                DateFormat("%d %B %Y %H:%M:%S", "01 July 2024 14:30:45"),
                DateFormat("%B %d, %Y %H:%M:%S", "July 01, 2024 14:30:45"),
                DateFormat("%Y-%m-%d %H:%M:%S%.f", "2024-07-01 14:30:45.123"),
                DateFormat("%d-%m-%Y %H:%M:%S%.f", "01-07-2024 14:30:45.123"),
                DateFormat("%m-%d-%Y %H:%M:%S%.f", "07-01-2024 14:30:45.123"),
                DateFormat("%d/%m/%Y %H:%M:%S%.f", "01/07/2024 14:30:45.123"),
                DateFormat("%m/%d/%Y %H:%M:%S%.f", "07/01/2024 14:30:45.123"),
                DateFormat("%d.%m.%Y %H:%M:%S%.f", "01.07.2024 14:30:45.123"),
                DateFormat("%m.%d.%Y %H:%M:%S%.f", "07.01.2024 14:30:45.123"),
                DateFormat("%Y.%m.%d %H:%M:%S%.f", "2024.07.01 14:30:45.123"),
                DateFormat("%Y/%m/%d %H:%M:%S%.f", "2024/07/01 14:30:45.123"),
                DateFormat("%d %B %Y %H:%M:%S%.f", "01 July 2024 14:30:45.123"),
                DateFormat("%B %d, %Y %H:%M:%S%.f", "July 01, 2024 14:30:45.123"),
                DateFormat("%Y-%m-%d", "2024-07-01"),
                DateFormat("%d-%m-%Y", "01-07-2024"),
                DateFormat("%m-%d-%Y", "07-01-2024"),
                DateFormat("%d/%m/%Y", "01/07/2024"),
                DateFormat("%m/%d/%Y", "07/01/2024"),
                DateFormat("%d.%m.%Y", "01.07.2024"),
                DateFormat("%m.%d.%Y", "07.01.2024"),
                DateFormat("%Y.%m.%d", "2024.07.01"),
                DateFormat("%Y/%m/%d", "2024/07/01"),
                DateFormat("%d %B %Y", "01 July 2024"),
                DateFormat("%B %d, %Y", "July 01, 2024"),
                DateFormat("%H:%M:%S", "14:30:45"),
                DateFormat("%I:%M:%S %p", "02:30:45 PM"),
                DateFormat("%H:%M", "14:30"),
                DateFormat("%I:%M %p", "02:30 PM"),
                DateFormat("%H:%M:%S%.f", "14:30:45.123"),
                DateFormat("%Y-%m-%dT%H:%M:%S%:z", "2024-07-01T14:30:45+00:00"),
                DateFormat(
                    "%a, %d %b %Y %H:%M:%S %z",
                    "Mon, 01 Jul 2024 14:30:45 +0000",
                ),
                DateFormat("%Y-%m-%dT%H:%M:%S%.f%:z", "2024-07-01T14:30:45.123+00:00"),
            ]
        }
    }
    
    pub(crate) trait ParseDate<'a> {
        fn parse(&self) -> Result<DateFormat<'a>, ParseError>;
    }
    
    impl<'a> ParseDate<'a> for String {
        fn parse(&self) -> Result<DateFormat<'a>, ParseError> {
            for pattern in DateFormat::patterns() {
                if let Ok(datetime) = NaiveDateTime::parse_from_str(self, pattern.0) {
                    return Ok(pattern)
                }
            }
    
            Err(ParseError::NotAvailable)
        }
    }
    
    pub enum ParseError {
        NotAvailable
    }
}

