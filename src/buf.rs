use crate::error::Error;

struct Buf<T: Copy> {
    items: Vec<Vec<T>>
}

impl<T: Copy> Buf<T> {
    fn get_cursor_at(&self, i: usize) -> Result<Cursor, Error> {
        todo!()
    }
}

struct Cursor {
    i_chunk: usize,
    i_item: usize
}