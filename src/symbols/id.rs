use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Clone)]
pub struct Id {
    pub string: Arc<String>,
}

impl Id {
    pub fn new(name: String) -> Id { Id { string: Arc::new(name) } }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl PartialEq<Self> for Id {
    fn eq(&self, other: &Self) -> bool { self.string == other.string }
}

impl PartialOrd<Self> for Id {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.string.cmp(&other.string))
    }
}

impl Eq for Id {}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> Ordering {
        self.string.cmp(&other.string)
    }
}
