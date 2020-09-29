use super::error::JustTextError;
use chrono::{DateTime, Utc};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

pub struct Metadatum {
    pub filename: String,
    pub created: DateTime<Utc>,
}

impl FromStr for Metadatum {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(Box::new(JustTextError::new("malformed metadatum")));
        }

        let filename = parts[0].to_string();
        let created = parts[1].parse::<DateTime<Utc>>()?;

        Ok(Metadatum { filename, created })
    }
}

impl fmt::Display for Metadatum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.filename, self.created.to_rfc3339())
    }
}
