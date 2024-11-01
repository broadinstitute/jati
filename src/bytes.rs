use std::io::Read;
use std::mem;
use std::sync::{Arc, RwLock};
use crate::error::Error;

const BUFFER_SIZE: usize = 1024;

#[derive(Clone)]
pub struct Bytes<R: Read> {
    pos: usize,
    buffer: Arc<ByteSlicables<R>>,
}

struct ByteSlicables<R: Read> {
    n_bytes: usize,
    buffer: Vec<u8>,
    slicing_box: RwLock<ByteSlicingBox<R>>,
}

enum ByteSlicing<R: Read> {
    Unsliced(R),
    Sliced(Arc<ByteSlicables<R>>),
    Empty,
}

struct ByteSlicingBox<R: Read> {
    slicing: ByteSlicing<R>,
}

impl<R: Read> Bytes<R> {
    pub fn new(mut read: R) -> Result<Self, Error> {
        let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];
        let n_bytes = read.read(&mut buffer)?;
        let slicing_box = RwLock::new(ByteSlicingBox::new(read));
        let buffer =
            Arc::new(ByteSlicables { n_bytes, buffer, slicing_box });
        Ok(Bytes { pos: 0, buffer })
    }
}

impl<R: Read> Iterator for Bytes<R> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.buffer.n_bytes {
            let byte = self.buffer.buffer[self.pos];
            self.pos += 1;
            Some(Ok(byte))
        } else {
            let next_slice =
                self.buffer.slicing_box.write().unwrap().next_slicable();
            match next_slice {
                Err(err) => Some(Err(err)),
                Ok(None) => None,
                Ok(Some(slicable)) => {
                    self.buffer = slicable;
                    self.pos = 0;
                    self.next()
                }
            }
        }
    }
}

impl<R: Read> ByteSlicingBox<R> {
    fn new(read: R) -> Self {
        ByteSlicingBox {
            slicing: ByteSlicing::Unsliced(read),
        }
    }
    fn next_slicable(&mut self) -> Result<Option<Arc<ByteSlicables<R>>>, Error> {
        match &self.slicing {
            ByteSlicing::Unsliced(_) => {
                let ByteSlicing::Unsliced(mut read) =
                    mem::replace(&mut self.slicing, ByteSlicing::Empty) else { unreachable!() };
                let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];
                let n_bytes = read.read(&mut buffer)?;
                if n_bytes == 0 {
                    Ok(None)
                } else {
                    let slicable = Arc::new(ByteSlicables {
                        n_bytes,
                        buffer,
                        slicing_box: RwLock::new(ByteSlicingBox::new(read)),
                    });
                    self.slicing = ByteSlicing::Sliced(slicable.clone());
                    Ok(Some(slicable))
                }
            }
            ByteSlicing::Sliced(slicables) => Ok(Some(slicables.clone())),
            ByteSlicing::Empty => Ok(None),
        }
    }
}