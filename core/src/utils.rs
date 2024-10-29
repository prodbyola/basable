use std::env::{self, VarError};

pub(crate) fn get_env(key: &str) -> Result<String, VarError> {
    env::var(key)
}

pub(crate) mod datetime_parser {
    use chrono::NaiveDateTime;

    /// An implementation of datetime pattern we can use in SQL queries. It's a turple struct with
    /// first item being the actual pattern and second item being a sample of the pattern.
    pub(crate) struct DatePattern<'a>(pub &'a str, pub &'a str);
    
    impl<'a> DatePattern<'a> {
        /// List of all supported [`DatePattern`]. We update this list if we need to support a new datetime pattern.
        pub fn supported() -> [DatePattern<'a>; 41] {
            [
                DatePattern("%Y-%m-%d %H:%M:%S", "2024-07-01 14:30:45"),
                DatePattern("%d-%m-%Y %H:%M:%S", "01-07-2024 14:30:45"),
                DatePattern("%m-%d-%Y %H:%M:%S", "07-01-2024 14:30:45"),
                DatePattern("%d/%m/%Y %H:%M:%S", "01/07/2024 14:30:45"),
                DatePattern("%m/%d/%Y %H:%M:%S", "07/01/2024 14:30:45"),
                DatePattern("%d.%m.%Y %H:%M:%S", "01.07.2024 14:30:45"),
                DatePattern("%m.%d.%Y %H:%M:%S", "07.01.2024 14:30:45"),
                DatePattern("%Y.%m.%d %H:%M:%S", "2024.07.01 14:30:45"),
                DatePattern("%Y/%m/%d %H:%M:%S", "2024/07/01 14:30:45"),
                DatePattern("%d %B %Y %H:%M:%S", "01 July 2024 14:30:45"),
                DatePattern("%B %d, %Y %H:%M:%S", "July 01, 2024 14:30:45"),
                DatePattern("%Y-%m-%d %H:%M:%S%.f", "2024-07-01 14:30:45.123"),
                DatePattern("%d-%m-%Y %H:%M:%S%.f", "01-07-2024 14:30:45.123"),
                DatePattern("%m-%d-%Y %H:%M:%S%.f", "07-01-2024 14:30:45.123"),
                DatePattern("%d/%m/%Y %H:%M:%S%.f", "01/07/2024 14:30:45.123"),
                DatePattern("%m/%d/%Y %H:%M:%S%.f", "07/01/2024 14:30:45.123"),
                DatePattern("%d.%m.%Y %H:%M:%S%.f", "01.07.2024 14:30:45.123"),
                DatePattern("%m.%d.%Y %H:%M:%S%.f", "07.01.2024 14:30:45.123"),
                DatePattern("%Y.%m.%d %H:%M:%S%.f", "2024.07.01 14:30:45.123"),
                DatePattern("%Y/%m/%d %H:%M:%S%.f", "2024/07/01 14:30:45.123"),
                DatePattern("%d %B %Y %H:%M:%S%.f", "01 July 2024 14:30:45.123"),
                DatePattern("%B %d, %Y %H:%M:%S%.f", "July 01, 2024 14:30:45.123"),
                DatePattern("%Y-%m-%d", "2024-07-01"),
                DatePattern("%d-%m-%Y", "01-07-2024"),
                DatePattern("%m-%d-%Y", "07-01-2024"),
                DatePattern("%d/%m/%Y", "01/07/2024"),
                DatePattern("%m/%d/%Y", "07/01/2024"),
                DatePattern("%d.%m.%Y", "01.07.2024"),
                DatePattern("%m.%d.%Y", "07.01.2024"),
                DatePattern("%Y.%m.%d", "2024.07.01"),
                DatePattern("%Y/%m/%d", "2024/07/01"),
                DatePattern("%d %B %Y", "01 July 2024"),
                DatePattern("%B %d, %Y", "July 01, 2024"),
                DatePattern("%H:%M:%S", "14:30:45"),
                DatePattern("%I:%M:%S %p", "02:30:45 PM"),
                DatePattern("%H:%M", "14:30"),
                DatePattern("%I:%M %p", "02:30 PM"),
                DatePattern("%H:%M:%S%.f", "14:30:45.123"),
                DatePattern("%Y-%m-%dT%H:%M:%S%:z", "2024-07-01T14:30:45+00:00"),
                DatePattern(
                    "%a, %d %b %Y %H:%M:%S %z",
                    "Mon, 01 Jul 2024 14:30:45 +0000",
                ),
                DatePattern("%Y-%m-%dT%H:%M:%S%.f%:z", "2024-07-01T14:30:45.123+00:00"),
            ]
        }
    
        /// The actual value of [`DatePattern`].
        pub fn value(&self) -> &'a str {
            self.0
        }

        /// An example of [`DatePattern`].
        pub fn example(&self) -> &'a str {
            self.1
        }
    }

    impl<'a> TryFrom<String> for DatePattern<'a> {
        type Error = ParseError;
    
        fn try_from(value: String) -> Result<Self, Self::Error> {
            for pattern in DatePattern::supported() {
                if let Ok(_) = NaiveDateTime::parse_from_str(&value, pattern.0) {
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

