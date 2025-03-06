default:
	cargo build

test:
	cargo test

install:
	cargo build --release
	sudo install target/release/libobs_portal_screencap.so /usr/lib/obs-plugins/