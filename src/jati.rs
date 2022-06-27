use std::sync::Arc;
use crate::buf::{Buf, BufBox};

pub struct Jati {
    buf: BufBox,
}

impl Jati {
    pub fn new(data: Vec<u8>) -> Jati {
        let buf = BufBox::new(Arc::new(Buf::new(vec!(data))));
        Jati { buf }
    }
    pub fn len_buffer(&self) -> usize { self.buf.len() }
}

