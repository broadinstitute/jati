use std::mem;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Bytes<S: AsRef<[u8]>, I: Iterator<Item=S>> {
    pos: usize,
    buffer: Arc<ByteSlicables<S, I>>,
}

struct ByteSlicables<S: AsRef<[u8]>, I: Iterator<Item=S>> {
    slicable: S,
    slicing_box: RwLock<ByteSlicingBox<S, I>>,
}

enum ByteSlicing<S: AsRef<[u8]>, I: Iterator<Item=S>> {
    Unsliced(I),
    Sliced(Arc<ByteSlicables<S, I>>),
    Empty,
}

struct ByteSlicingBox<S: AsRef<[u8]>, I: Iterator<Item=S>> {
    slicing: ByteSlicing<S, I>,
}

impl<S: AsRef<[u8]>, I: Iterator<Item=S>> Bytes<S, I> {
    pub fn new(mut slicables: I) -> Self {
        let slicable = slicables.next().unwrap();
        let slicing_box = RwLock::new(ByteSlicingBox::new(slicables));
        let buffer = Arc::new(ByteSlicables { slicable, slicing_box });
        Bytes { pos: 0, buffer }
    }
}

impl<S: AsRef<[u8]>, I: Iterator<Item=S>> Iterator for Bytes<S, I> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = self.buffer.slicable.as_ref();
        if self.pos < slice.len() {
            let byte = slice[self.pos];
            self.pos += 1;
            Some(byte)
        } else {
            let next_slice =
                self.buffer.slicing_box.write().unwrap().next_slicable();
            match next_slice {
                None => None,
                Some(slicable) => {
                    self.buffer = slicable;
                    self.pos = 0;
                    self.next()
                }
            }
        }
    }
}

impl<S: AsRef<[u8]>, I: Iterator<Item=S>> ByteSlicingBox<S, I> {
    fn new(iterator: I) -> Self {
        ByteSlicingBox {
            slicing: ByteSlicing::Unsliced(iterator),
        }
    }
    fn next_slicable(&mut self) -> Option<Arc<ByteSlicables<S, I>>> {
        match &self.slicing {
            ByteSlicing::Unsliced(_) => {
                let ByteSlicing::Unsliced(mut iterator) =
                    mem::replace(&mut self.slicing, ByteSlicing::Empty) else { unreachable!() };
                match iterator.next() {
                    None => { None }
                    Some(slice) => {
                        let slicable = Arc::new(ByteSlicables {
                            slicable: slice,
                            slicing_box: RwLock::new(ByteSlicingBox::new(iterator)),
                        });
                        self.slicing = ByteSlicing::Sliced(slicable.clone());
                        Some(slicable)
                    }
                }
            }
            ByteSlicing::Sliced(slicables) => Some(slicables.clone()),
            ByteSlicing::Empty => None,
        }
    }
}