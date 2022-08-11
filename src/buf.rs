use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub(crate) struct Pos {}

pub(crate) struct Buf {}

pub(crate) struct BufBox {
    buf: Rc<Buf>,
}

impl Buf {
    pub(crate) fn get_len_sums(chunks: &Vec<Vec<u8>>) -> Vec<usize> {
        let mut len_sums: Vec<usize> = Vec::new();
        let mut len_sum: usize = 0;
        for chunk in chunks {
            len_sum += chunk.len();
            len_sums.push(len_sum);
        }
        len_sums
    }
    pub(crate) fn new() -> Buf {
        Buf {}
    }
}

impl BufBox {
    pub(crate) fn new(buf: Rc<Buf>) -> BufBox {
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
