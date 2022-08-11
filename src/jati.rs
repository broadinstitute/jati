use std::rc::Rc;
use crate::buf::{Buf, BufBox};

pub(crate) struct Jati {
    buf: BufBox,
}

impl Jati {
    fn new(buf: BufBox) -> Jati {
        Jati { buf }
    }
}

impl From<Vec<u8>> for Jati {
    fn from(data: Vec<u8>) -> Self {
        let buf = BufBox::new(Rc::new(Buf::new()));
        Jati::new(buf)
    }
}

impl From<String> for Jati {
    fn from(string: String) -> Self {
        Jati::from(string.into_bytes())
    }
}

