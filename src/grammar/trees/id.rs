pub struct Id {
    pub string: String
}

impl Id {
    pub fn new(name: String) -> Id { Id { string: name } }
}