use crate::errors::NetError;
use crate::ethernet::{ethernet_rx, MacAddress};
use crate::ipv4::Ipv4Address;
use crate::mbuf::Mbuf;

pub struct NetworkInterface {
    nic: tun_tap::Iface,
    pub mac_address: MacAddress,
    pub ipv4_address: Ipv4Address,
}

impl NetworkInterface {
    pub fn new(mac_address: MacAddress, ipv4_address: Ipv4Address) -> Self {
        NetworkInterface {
            nic: tun_tap::Iface::without_packet_info("tap0", tun_tap::Mode::Tap)
                .expect("Failed to create TAP device"),
            mac_address: mac_address,
            ipv4_address: ipv4_address,
        }
    }

    pub fn recv(&self) -> Result<(), NetError<'static>> {
        let mut mbuf = Mbuf::new();

        let n = self.nic.recv(mbuf.get_mut_ref())?;

        mbuf.set_len(n);

        ethernet_rx(&self, &mut mbuf)
    }

    pub fn send(&self, mbuf: &mut Mbuf) -> Result<(), NetError<'static>> {
        let _ = self.nic.send(mbuf.get_mut_ref())?;

        Ok(())
    }
}
