mod arp;
mod errors;
mod ethernet;
mod interface;
mod ipv4;
mod mbuf;

use errors::NetError;
use ethernet::MacAddress;
use interface::NetworkInterface;
use ipv4::Ipv4Address;

fn main() -> Result<(), NetError<'static>> {
    // DEBUG
    let mac_address = MacAddress::from_slice(&[0xAB, 0xAB, 0xAB, 0xAB, 0xAB, 0xAB]);
    let ipv4_address = Ipv4Address::from_slice(&[10, 0, 0, 2]);

    let interface = NetworkInterface::new(mac_address, ipv4_address);

    loop {
        if let Err(err) = interface.recv() {
            println!("{}", err);
        }
    }
}
