use crate::error::Error;

struct Buf<T: Copy> {
    items: Vec<Vec<T>>,
}

impl<T: Copy> Buf<T> {
    fn get_cursor_at(&self, i: usize) -> Result<Cursor<T>, Error> {
        let mut i_chunk: usize = 0;
        let mut i_item: usize = i;
        loop {
            match self.items.get(i_chunk) {
                Some(chunk) => {
                    if i_item < chunk.len() {
                        Ok(Cursor::new(self, i_chunk, i_item))
                    }
                    todo!()
                }
                None => { todo!() }
            }
        }
    }
}

struct Cursor<'a, T: Copy> {
    buf: &'a Buf<T>,
    i_chunk: usize,
    i_item: usize,
}

impl Cursor<T> {
    pub(crate) fn new(buf: &Buf<T>, i_chunk: usize, i_item: usize) -> Cursor<T> {
        Cursor { buf, i_chunk, i_item}
    }
}