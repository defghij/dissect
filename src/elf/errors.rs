use std::fmt;

//Errors

#[derive(Debug)]
pub struct InvalidHeaderError {
  details: String
}

impl InvalidHeaderError {
    pub fn new(msg: &str) -> InvalidHeaderError {
        InvalidHeaderError { details: msg.to_string()}
    }
}

impl fmt::Display for InvalidHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

/*impl Error for InvalidHeaderError {
    fn description(&self) -> &str {
        &self.details
    }
}*/
