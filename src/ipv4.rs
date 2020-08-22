use std::fmt;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Ipv4Address([u8; 4]);

impl Ipv4Address {
    pub const LENGTH: usize = 4;

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&slice[..4]);
        Ipv4Address(buf)
    }

    pub fn to_slice(&self, slice: &mut [u8]) {
        slice.copy_from_slice(&self.0);
    }
}

impl fmt::Display for Ipv4Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ipv4_address_str = format!("{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3]);
        write!(f, "{}", ipv4_address_str)
    }
}
