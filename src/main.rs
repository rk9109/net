mod mbuf;

use std::io;

use mbuf::Mbuf;

fn main() -> Result<(), io::Error> {
    let nic = tun_tap::Iface::without_packet_info("tap0", tun_tap::Mode::Tap)?;

    loop {
        let mut mbuf = Mbuf::new();

        let n = nic.recv(mbuf.get_mut_ref())?;

        mbuf.set_len(n);
    }
}
