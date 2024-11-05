use crate::error::Error;
use std::io::Read;
use std::mem;
use std::sync::{Arc, RwLock};

const BUFFER_SIZE: usize = 1024;

#[derive(Clone)]
pub struct Bytes<R: Read> {
    pos: usize,
    slices: Arc<BytesSlice<R>>,
}
struct BytesSlice<R: Read> {
    n_bytes: usize,
    buffer: Vec<u8>,
    slice_gen: RwLock<ByteSlicingBox<R>>,
}
struct ByteSlicingBox<R: Read> {
    slicing: ByteSlicing<R>,
}
enum ByteSlicing<R: Read> {
    Unsliced(R),
    Sliced(Result<Option<Arc<BytesSlice<R>>>, Error>),
}

impl<R: Read> Bytes<R> {
    pub fn new(mut read: R) -> Result<Self, Error> {
        let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];
        let n_bytes = read.read(&mut buffer)?;
        let slice_gen = RwLock::new(ByteSlicingBox::new(read));
        let buffer = Arc::new(BytesSlice { n_bytes, buffer, slice_gen });
        Ok(Bytes { pos: 0, slices: buffer })
    }
}

impl<R: Read> Iterator for Bytes<R> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.slices.n_bytes {
            let byte = self.slices.buffer[self.pos];
            self.pos += 1;
            Some(Ok(byte))
        } else {
            match self.slices.slice_gen.write().unwrap().next_slicable() {
                Err(err) => Some(Err(err)),
                Ok(None) => None,
                Ok(Some(slicable)) => {
                    self.slices = slicable;
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
    fn next_slicable(&mut self) -> Result<Option<Arc<BytesSlice<R>>>, Error> {
        match &self.slicing {
            ByteSlicing::Unsliced(_) => {
                let empty_slicing = ByteSlicing::Sliced(Ok(None));
                let ByteSlicing::Unsliced(mut read) =
                    mem::replace(&mut self.slicing, empty_slicing) else { unreachable!() };
                let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];
                match read.read(&mut buffer) {
                    Ok(n_bytes) => {
                        if n_bytes == 0 {
                            self.slicing = ByteSlicing::Sliced(Ok(None));
                            Ok(None)
                        } else {
                            let slicable = Arc::new(BytesSlice {
                                n_bytes,
                                buffer,
                                slice_gen: RwLock::new(ByteSlicingBox::new(read)),
                            });
                            self.slicing = ByteSlicing::Sliced(Ok(Some(slicable.clone())));
                            Ok(Some(slicable))
                        }
                    }
                    Err(err) => {
                        let error = Error::from(err);
                        self.slicing = ByteSlicing::Sliced(Err(error.clone()));
                        Err(error)
                    },
                }
            }
            ByteSlicing::Sliced(slicables) => {
                match slicables {
                    Ok(None) => Ok(None),
                    Ok(Some(slicable)) => Ok(Some(slicable.clone())),
                    Err(err) => Err(err.clone()),
                }
            },
        }
    }
}