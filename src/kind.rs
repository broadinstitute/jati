use std::rc::Rc;

pub(crate) struct Kind {
    name: Rc<String>
}

impl Kind {
    pub(crate) fn new(name: String) -> Kind {
        let name = Rc::new(name);
        Kind { name }
    }
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
        Rc::ptr_eq(&self.name, &other.name)
    }
}

impl Eq for Kind {

}