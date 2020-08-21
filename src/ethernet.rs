use std::fmt;

use byteorder::{ByteOrder, NetworkEndian};

use crate::arp::arp_rx;
use crate::errors::NetError;
use crate::mbuf::Mbuf;

#[derive(Debug, Eq, PartialEq)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut arr = [0u8; 6];
        arr.copy_from_slice(&slice[..6]);
        MacAddress(arr)
    }

    pub fn to_slice(&self) -> &[u8] {
        todo!();
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mac_address_str = format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        );
        write!(f, "{}", mac_address_str)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum EthernetType {
    Ipv4 = 0x0800,
    Ipv6 = 0x86DD,
    Arp = 0x0806,
    Unsupported,
}

impl EthernetType {
    pub fn from_slice(slice: &[u8]) -> Self {
        let ethernet_type = NetworkEndian::read_u16(slice);

        match ethernet_type {
            0x0800 => EthernetType::Ipv4,
            0x86DD => EthernetType::Ipv6,
            0x0806 => EthernetType::Arp,
            _ => EthernetType::Unsupported,
        }
    }

    pub fn to_slice(&self) -> &[u8] {
        todo!();
    }
}

#[derive(Debug)]
pub struct EthernetHeader {
    src: MacAddress,             // Source MAC address
    dest: MacAddress,            // Destination MAC address
    ethernet_type: EthernetType, // Ethernet type
}

impl EthernetHeader {
    const LENGTH: usize = 14;

    pub fn pull(mbuf: &mut Mbuf) -> Result<Self, NetError<'static>> {
        let buf = mbuf
            .pull(EthernetHeader::LENGTH)
            .ok_or_else(|| NetError::ParseError("Invalid ethernet header"))?;

        Ok(EthernetHeader {
            src: MacAddress::from_slice(&buf[..6]),
            dest: MacAddress::from_slice(&buf[6..12]),
            ethernet_type: EthernetType::from_slice(&buf[12..14]),
        })
    }

    pub fn push(
        mut mbuf: Mbuf,
        src: MacAddress,
        dest: MacAddress,
        ethernet_type: EthernetType,
    ) -> Result<(), NetError<'static>> {
        todo!();
    }
}

impl fmt::Display for EthernetHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "src: {} - dest: {} - pro: {:?}",
            self.src, self.dest, self.ethernet_type
        )
    }
}

pub fn ethernet_rx(mbuf: &mut Mbuf) -> Result<(), NetError<'static>> {
    let ethernet_header = EthernetHeader::pull(mbuf)?;

    // DEBUG
    println!("{}", ethernet_header);

    match ethernet_header.ethernet_type {
        EthernetType::Ipv4 => {
            return Err(NetError::UnsupportedError("Ipv4 not implemented"));
        }
        EthernetType::Ipv6 => {
            return Err(NetError::UnsupportedError("Ipv6 not implemented"));
        }
        EthernetType::Arp => {
            return arp_rx(mbuf);
        }
        EthernetType::Unsupported => {
            return Err(NetError::UnsupportedError("Unsupported protocol"));
        }
    }
}
