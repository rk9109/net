use std::fmt;

use byteorder::{ByteOrder, NetworkEndian};

use crate::arp::arp_rx;
use crate::errors::NetError;
use crate::interface::NetworkInterface;
use crate::mbuf::Mbuf;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
    pub const LENGTH: usize = 6;

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut buf = [0u8; 6];
        buf.copy_from_slice(&slice[..6]);
        MacAddress(buf)
    }

    pub fn to_slice(&self, slice: &mut [u8]) {
        slice.copy_from_slice(&self.0);
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
    Ipv4,
    Ipv6,
    Arp,
    Unsupported(u16),
}

impl EthernetType {
    pub fn from_slice(slice: &[u8]) -> Self {
        let ethernet_type = NetworkEndian::read_u16(slice);

        match ethernet_type {
            0x0800 => EthernetType::Ipv4,
            0x86DD => EthernetType::Ipv6,
            0x0806 => EthernetType::Arp,
            _ => EthernetType::Unsupported(ethernet_type),
        }
    }

    pub fn to_slice(&self, slice: &mut [u8]) {
        match self {
            EthernetType::Ipv4 => NetworkEndian::write_u16(slice, 0x0800),
            EthernetType::Ipv6 => NetworkEndian::write_u16(slice, 0x86DD),
            EthernetType::Arp => NetworkEndian::write_u16(slice, 0x0806),
            EthernetType::Unsupported(ethernet_type) => {
                NetworkEndian::write_u16(slice, *ethernet_type)
            }
        }
    }
}

#[derive(Debug)]
pub struct EthernetHeader {
    src: MacAddress,             // Source MAC address
    dest: MacAddress,            // Destination MAC address
    ethernet_type: EthernetType, // Ethernet type
}

impl EthernetHeader {
    pub const LENGTH: usize = 14;

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
        mbuf: &mut Mbuf,
        src: MacAddress,
        dest: MacAddress,
        ethernet_type: EthernetType,
    ) -> Result<(), NetError<'static>> {
        let buf = mbuf
            .push(EthernetHeader::LENGTH)
            .ok_or_else(|| NetError::ParseError("Invalid ethernet header"))?;

        src.to_slice(&mut buf[..6]);
        dest.to_slice(&mut buf[6..12]);
        ethernet_type.to_slice(&mut buf[12..14]);

        Ok(())
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

pub fn ethernet_tx(
    interface: &NetworkInterface,
    mbuf: &mut Mbuf,
    src: MacAddress,
    dest: MacAddress,
    ethernet_type: EthernetType,
) -> Result<(), NetError<'static>> {
    EthernetHeader::push(mbuf, src, dest, ethernet_type)?;

    interface.send(mbuf)
}

pub fn ethernet_rx(interface: &NetworkInterface, mbuf: &mut Mbuf) -> Result<(), NetError<'static>> {
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
            return arp_rx(interface, mbuf);
        }
        EthernetType::Unsupported(_) => {
            return Err(NetError::UnsupportedError("Unsupported protocol"));
        }
    }
}
