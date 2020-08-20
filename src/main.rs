mod errors;
mod ethernet;
mod mbuf;

use errors::NetError;
use ethernet::ethernet_rx;
use mbuf::Mbuf;

fn main() -> Result<(), NetError<'static>> {
    let nic = tun_tap::Iface::without_packet_info("tap0", tun_tap::Mode::Tap)?;

    loop {
        let mut mbuf = Mbuf::new();

        let n = nic.recv(mbuf.get_mut_ref())?;

        mbuf.set_len(n);

        if let Err(err) = ethernet_rx(mbuf) {
            println!("{}", err);
        }
    }
}
