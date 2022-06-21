use crate::buf::Buf;

pub struct Jati {
    buf: Buf<u8>
}

impl Jati {
    pub fn new(data: Vec<u8>) -> Jati {
        let buf = Buf::new(vec!(data));
        Jati { buf }
    }
}