use std::ops::Deref;
use std::sync::Arc;

struct Pos {
    i_chunk: usize,
    i_in_chunk: usize
}

pub struct Buf {
    chunks: Vec<Vec<u8>>
}

pub(crate) struct BufBox {
    buf: Arc<Buf>
}

impl Buf {
    pub(crate) fn new(chunks: Vec<Vec<u8>>) -> Buf {
        Buf { chunks }
    }
    pub(crate) fn len(&self) -> usize {
        todo!()
    }
}

impl BufBox {
    pub(crate) fn new(buf: Arc<Buf>) -> BufBox {
        BufBox { buf }
    }
}

impl Clone for BufBox {
    fn clone(&self) -> Self {
        let buf = self.buf.clone();
        BufBox { buf }
    }
}

impl Deref for BufBox {
    type Target = Buf;

    fn deref(&self) -> &Self::Target {
        &*self.buf
    }
}
