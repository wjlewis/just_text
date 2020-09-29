use std::borrow::Cow;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct JustTextError<'a> {
    message: Cow<'a, str>,
}

impl<'a> JustTextError<'a> {
    pub fn new<S>(message: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        JustTextError {
            message: message.into(),
        }
    }
}

impl<'a> fmt::Display for JustTextError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'a> Error for JustTextError<'a> {}
