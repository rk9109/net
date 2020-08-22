IP_ADDRESS = 10.0.0.1/24
MAC_ADDRESS = 32:32:60:AB:2F:01

default: net run

net:
	cargo build --release

run:
	target/release/net

setup:
	setcap cap_net_admin=eip target/release/net
	ip addr add $(IP_ADDRESS) dev tap0
	ip link set tap0 addr $(MAC_ADDRESS)
	ip link set up tap0

clean:
	cargo clean
