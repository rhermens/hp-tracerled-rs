.PHONY: build install

build: 
	cargo build --release
install:
	install -m 755 ./target/release/hp-tracerled /usr/local/bin/hp-tracerled
	install -m 644 ./systemd/hp-tracerled.service /etc/systemd/system/hp-tracerled.service
	systemctl daemon-reload
	systemctl enable hp-tracerled.service

