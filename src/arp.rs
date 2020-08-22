use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

use byteorder::{ByteOrder, NetworkEndian};
use lazy_static::lazy_static;

use crate::errors::NetError;
use crate::ethernet::{ethernet_tx, EthernetHeader, EthernetType, MacAddress};
use crate::interface::NetworkInterface;
use crate::ipv4::Ipv4Address;
use crate::mbuf::Mbuf;

lazy_static! {
    static ref ARP_CACHE: ArpCache = ArpCache::new();
}

#[derive(Debug, Eq, PartialEq)]
pub enum ArpHardwareType {
    Ethernet,
    Unsupported(u16),
}

impl ArpHardwareType {
    pub fn from_slice(slice: &[u8]) -> Self {
        let arp_hardware_type = NetworkEndian::read_u16(slice);

        match arp_hardware_type {
            0x0001 => ArpHardwareType::Ethernet,
            _ => ArpHardwareType::Unsupported(arp_hardware_type),
        }
    }

    pub fn to_slice(&self, slice: &mut [u8]) {
        match self {
            ArpHardwareType::Ethernet => NetworkEndian::write_u16(slice, 0x0001),
            ArpHardwareType::Unsupported(arp_hardware_type) => {
                NetworkEndian::write_u16(slice, *arp_hardware_type)
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ArpOpcode {
    Request,
    Reply,
    Unsupported(u16),
}

impl ArpOpcode {
    pub fn from_slice(slice: &[u8]) -> Self {
        let arp_opcode = NetworkEndian::read_u16(slice);

        match arp_opcode {
            0x0001 => ArpOpcode::Request,
            0x0002 => ArpOpcode::Reply,
            _ => ArpOpcode::Unsupported(arp_opcode),
        }
    }

    pub fn to_slice(&self, slice: &mut [u8]) {
        match self {
            ArpOpcode::Request => NetworkEndian::write_u16(slice, 0x0001),
            ArpOpcode::Reply => NetworkEndian::write_u16(slice, 0x0002),
            ArpOpcode::Unsupported(arp_opcode) => NetworkEndian::write_u16(slice, *arp_opcode),
        }
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
    pub const LENGTH: usize = 28;

    pub fn pull(mbuf: &mut Mbuf) -> Result<Self, NetError<'static>> {
        let buf = mbuf
            .pull(ArpHeader::LENGTH)
            .ok_or_else(|| NetError::ParseError("Invalid arp header"))?;

        let hrd = match ArpHardwareType::from_slice(&buf[..2]) {
            ArpHardwareType::Unsupported(_) => {
                return Err(NetError::UnsupportedError("Unsupported arp hardware"));
            }
            hrd => hrd,
        };

        let pro = match EthernetType::from_slice(&buf[2..4]) {
            EthernetType::Unsupported(_) => {
                return Err(NetError::UnsupportedError("Unsupported arp protocol"));
            }
            pro => pro,
        };

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

    pub fn push(
        mbuf: &mut Mbuf,
        op: ArpOpcode,
        sha: MacAddress,
        spa: Ipv4Address,
        tha: MacAddress,
        tpa: Ipv4Address,
    ) -> Result<(), NetError<'static>> {
        let buf = mbuf
            .push(ArpHeader::LENGTH)
            .ok_or_else(|| NetError::ParseError("Invalid arp header"))?;

        let hrd = ArpHardwareType::Ethernet;
        hrd.to_slice(&mut buf[0..2]);

        let pro = EthernetType::Ipv4;
        pro.to_slice(&mut buf[2..4]);

        buf[4] = MacAddress::LENGTH as u8;
        buf[5] = Ipv4Address::LENGTH as u8;

        op.to_slice(&mut buf[6..8]);
        sha.to_slice(&mut buf[8..14]);
        spa.to_slice(&mut buf[14..18]);
        tha.to_slice(&mut buf[18..24]);
        tpa.to_slice(&mut buf[24..28]);

        Ok(())
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

struct ArpCache {
    map: Mutex<HashMap<Ipv4Address, MacAddress>>,
}

impl ArpCache {
    pub fn new() -> Self {
        ArpCache {
            map: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&mut self, ipv4_address: Ipv4Address) -> Option<MacAddress> {
        let map = self.map.lock().unwrap();
        map.get(&ipv4_address).map(|v| *v)
    }

    pub fn put(&self, ipv4_address: Ipv4Address, mac_address: MacAddress) {
        let mut map = self.map.lock().unwrap();
        map.insert(ipv4_address, mac_address);
    }

    pub fn merge(&self, ipv4_address: Ipv4Address, mac_address: MacAddress) -> bool {
        let mut map = self.map.lock().unwrap();
        if map.contains_key(&ipv4_address) {
            map.insert(ipv4_address, mac_address);
            return true;
        }
        false
    }
}

fn arp_tx(
    interface: &NetworkInterface,
    mbuf: &mut Mbuf,
    arp_header: ArpHeader,
) -> Result<(), NetError<'static>> {
    ArpHeader::push(
        mbuf,
        ArpOpcode::Reply,
        interface.mac_address,
        interface.ipv4_address,
        arp_header.sha,
        arp_header.spa,
    )?;

    ethernet_tx(
        interface,
        mbuf,
        interface.mac_address,
        arp_header.sha,
        EthernetType::Arp,
    )
}

pub fn arp_rx(interface: &NetworkInterface, mbuf: &mut Mbuf) -> Result<(), NetError<'static>> {
    let arp_header = ArpHeader::pull(mbuf)?;

    // DEBUG
    println!("{}", arp_header);

    let merge = ARP_CACHE.merge(arp_header.spa, arp_header.sha);

    if arp_header.tpa != interface.ipv4_address {
        return Ok(());
    }

    if !merge {
        ARP_CACHE.put(arp_header.spa, arp_header.sha);
    }

    match arp_header.op {
        ArpOpcode::Request => {
            let mut mbuf_r = Mbuf::new();

            mbuf_r.set_len(0);
            mbuf_r.set_offset(EthernetHeader::LENGTH + ArpHeader::LENGTH);

            return arp_tx(interface, &mut mbuf_r, arp_header);
        }
        ArpOpcode::Reply => {
            return Ok(());
        }
        ArpOpcode::Unsupported(_) => {
            return Err(NetError::UnsupportedError("Unsupported arp opcode"));
        }
    }
}
