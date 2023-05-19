use std::fmt::{Display, Formatter};


pub struct Id {
    pub string: String,
}

impl Id {
    pub fn new(name: String) -> Id { Id { string: name } }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl Clone for Id {
    fn clone(&self) -> Self {
        let string = self.string.clone();
        Id { string }
    }
}