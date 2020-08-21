use std::fmt;

use byteorder::{ByteOrder, NetworkEndian};

use crate::errors::NetError;
use crate::ethernet::{EthernetType, MacAddress};
use crate::ipv4::Ipv4Address;
use crate::mbuf::Mbuf;

#[derive(Debug, Eq, PartialEq)]
pub enum ArpHardwareType {
    Ethernet = 0x0001,
    Unsupported,
}

impl ArpHardwareType {
    pub fn from_slice(slice: &[u8]) -> Self {
        let arp_hardware_type = NetworkEndian::read_u16(slice);

        match arp_hardware_type {
            0x0001 => ArpHardwareType::Ethernet,
            _ => ArpHardwareType::Unsupported,
        }
    }

    pub fn to_slice(&self) -> &[u8] {
        todo!();
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ArpOpcode {
    Request = 0x0001,
    Reply = 0x0002,
    Unsupported,
}

impl ArpOpcode {
    pub fn from_slice(slice: &[u8]) -> Self {
        let arp_opcode = NetworkEndian::read_u16(slice);

        match arp_opcode {
            0x0001 => ArpOpcode::Request,
            0x0002 => ArpOpcode::Reply,
            _ => ArpOpcode::Unsupported,
        }
    }

    pub fn to_slice(&self) -> &[u8] {
        todo!();
    }
}

#[derive(Debug)]
pub struct ArpHeader {
    hrd: ArpHardwareType, // Hardware type
    pro: EthernetType,    // Protocol type
    hln: u8,              // Hardware address length
    pln: u8,              // Protocol address length
    op: ArpOpcode,        // Opcode
    sha: MacAddress,      // Sender MAC address
    spa: Ipv4Address,     // Sender Ipv4 address
    tha: MacAddress,      // Target MAC address
    tpa: Ipv4Address,     // Target Ipv4 address
}

impl ArpHeader {
    const LENGTH: usize = 28;

    pub fn pull(mbuf: &mut Mbuf) -> Result<Self, NetError<'static>> {
        let buf = mbuf
            .pull(ArpHeader::LENGTH)
            .ok_or_else(|| NetError::ParseError("Invalid arp header"))?;

        let hrd = ArpHardwareType::from_slice(&buf[..2]);
        if hrd == ArpHardwareType::Unsupported {
            return Err(NetError::UnsupportedError("Unsupported arp hardware"));
        }

        let pro = EthernetType::from_slice(&buf[2..4]);
        if pro == EthernetType::Unsupported {
            return Err(NetError::UnsupportedError("Unsupported arp protocol"));
        }

        Ok(ArpHeader {
            hrd: hrd,
            pro: pro,
            hln: buf[4],
            pln: buf[5],
            op: ArpOpcode::from_slice(&buf[6..8]),
            sha: MacAddress::from_slice(&buf[8..14]),
            spa: Ipv4Address::from_slice(&buf[14..18]),
            tha: MacAddress::from_slice(&buf[18..24]),
            tpa: Ipv4Address::from_slice(&buf[24..28]),
        })
    }
}

impl fmt::Display for ArpHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "op: {:?} - sha: {} - spa: {} - tha: {} - tpa: {}",
            self.op, self.sha, self.spa, self.tha, self.tpa
        )
    }
}

pub fn arp_rx(mbuf: &mut Mbuf) -> Result<(), NetError<'static>> {
    let arp_header = ArpHeader::pull(mbuf)?;

    // DEBUG
    println!("{}", arp_header);

    Ok(())
}
