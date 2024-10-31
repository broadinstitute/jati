use std::sync::Arc;

pub struct Bytes {
    pos: usize,
}
struct BytesSlicables<S: AsRef<[u8]>, I: Iterator<Item=S>> {
    slicable: Arc<S>,
    iter: I,
}

