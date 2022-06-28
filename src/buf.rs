mod cursor;

use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Copy)]
struct Pos {
    i_chunk: usize,
    i_in_chunk: usize,
}

pub struct Buf {
    chunks: Vec<Vec<u8>>,
    len_sums: Vec<usize>,
}

pub(crate) struct BufBox {
    buf: Arc<Buf>,
}

impl Buf {
    fn get_len_sums(chunks: &Vec<Vec<u8>>) -> Vec<usize> {
        let mut len_sums: Vec<usize> = Vec::new();
        let mut len_sum: usize = 0;
        for chunk in chunks {
            len_sum += chunk.len();
            len_sums.push(len_sum);
        }
        len_sums
    }
    pub(crate) fn new(chunks: Vec<Vec<u8>>) -> Buf {
        let len_sums = Buf::get_len_sums(&chunks);
        Buf { chunks, len_sums }
    }
    pub(crate) fn len(&self) -> usize {
        *self.len_sums.last().unwrap_or(&0usize)
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
