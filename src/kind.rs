use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub(crate) struct Kind {
    name: Rc<String>
}

impl Kind {
    pub(crate) fn new(name: String) -> Kind {
        let name = Rc::new(name);
        Kind { name }
    }
    pub(crate) fn as_str(&self) -> &str { self.name.as_str() }
}

impl From<&str> for Kind {
    fn from(string: &str) -> Self {
        Kind::new(String::from(string))
    }
}

impl Clone for Kind {
    fn clone(&self) -> Self {
        let name = self.name.clone();
        Kind { name }
    }
}

impl PartialEq<Self> for Kind {
    fn eq(&self, other: &Self) -> bool {
        self.name.as_str() == other.name.as_str()
    }
}

impl Eq for Kind {

}

impl PartialOrd<Self> for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}