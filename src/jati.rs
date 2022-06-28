use std::sync::Arc;
use crate::buf::{Buf, BufBox};

pub struct Jati {
    buf: BufBox,
}

impl Jati {
    fn new(buf: BufBox) -> Jati {
        Jati { buf }
    }
    pub fn len_buffer(&self) -> usize { self.buf.len() }
}

impl From<Vec<u8>> for Jati {
    fn from(data: Vec<u8>) -> Self {
        let buf = BufBox::new(Arc::new(Buf::new(vec!(data))));
        Jati::new(buf)
    }
}

impl From<String> for Jati {
    fn from(string: String) -> Self {
        Jati::from(string.into_bytes())
    }
}

