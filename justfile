run:
	cargo r -r

sync-mold-path:
	echo "[target.x86_64-unknown-linux-gnu]" > .cargo/config.toml
	if [ -z $(which mold) ]; then echo "mold not found"; exit 1; fi
	echo "rustflags = [\"-C\", \"link-arg=-fuse-ld=$(which mold)\"]" >> .cargo/config.toml