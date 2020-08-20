pub struct Mbuf {
    b: Box<[u8]>,
    len: usize,
    offset: usize,
}

impl Mbuf {
    const SIZE: usize = 1500;

    pub fn new() -> Self {
        Mbuf {
            b: Box::new([0u8; Mbuf::SIZE]),
            len: Mbuf::SIZE,
            offset: 0,
        }
    }

    pub fn get_mut_ref(&mut self) -> &mut [u8] {
        &mut self.b
    }

    pub fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    pub fn pull(&mut self, len: usize) -> Option<&mut [u8]> {
        let slice = self.b.get_mut(self.offset..(self.offset + len));

        if slice.is_some() {
            self.len -= len;
            self.offset += len;
        }

        slice
    }

    pub fn push(&mut self, len: usize) -> Option<&mut [u8]> {
        let slice = self.b.get_mut((self.offset - len)..self.offset);

        if slice.is_some() {
            self.len += len;
            self.offset -= len;
        }

        slice
    }
}
