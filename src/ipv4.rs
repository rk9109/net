use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Ipv4Address([u8; 4]);

impl Ipv4Address {
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut arr = [0u8; 4];
        arr.copy_from_slice(&slice[..4]);
        Ipv4Address(arr)
    }

    pub fn to_slice(&self) -> &[u8] {
        todo!();
    }
}

impl fmt::Display for Ipv4Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ipv4_address_str = format!("{}.{}.{}.{}", self.0[0], self.0[1], self.0[2], self.0[3]);
        write!(f, "{}", ipv4_address_str)
    }
}
