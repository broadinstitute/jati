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
