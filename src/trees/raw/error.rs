use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct TreeError {
    message: String
}

impl TreeError {
    pub(crate) fn new(message: String) -> TreeError {
        TreeError { message }
    }
}

impl Debug for TreeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}" , self.message)
    }
}

impl Display for TreeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}" , self.message)
    }
}

impl Error for TreeError {

}