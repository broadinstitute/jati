use crate::error::{Error, ErrorKind};

pub(crate) struct Buf<T: Copy> {
    items: Vec<Vec<T>>,
}

impl<T: Copy> Buf<T> {
    pub(crate) fn new(items: Vec<Vec<T>>) -> Buf<T> {
        Buf { items }
    }
    fn get_cursor_at(&self, i: usize) -> Result<Cursor<T>, Error> {
        let mut i_chunk: usize = 0;
        let mut i_item: usize = i;
        loop {
            match self.items.get(i_chunk) {
                Some(chunk) => {
                    if i_item < chunk.len() {
                        break Ok(Cursor::new(self, i_chunk, i_item))
                    } else {
                        i_chunk += 1;
                        i_item -= chunk.len();
                    }
                }
                None => {
                    break Err(Error::new(ErrorKind::OutOfRange,
                                         format!("{} is out of range", i)))
                }
            }
        }
    }
}

struct Cursor<'a, T: Copy> {
    buf: &'a Buf<T>,
    i_chunk: usize,
    i_item: usize,
}

impl<'a, T: Copy> Cursor<'a, T> {
    pub(crate) fn new(buf: &Buf<T>, i_chunk: usize, i_item: usize) -> Cursor<T> {
        Cursor { buf, i_chunk, i_item}
    }
}